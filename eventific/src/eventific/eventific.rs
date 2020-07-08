use crate::store::{Store, MemoryStore, StoreError};
use crate::EventificBuilder;
use std::marker::PhantomData;
use crate::aggregate::{StateBuilder, Aggregate};
use crate::eventific::EventificError;
use futures::{TryFutureExt, TryStreamExt, StreamExt};
use std::future::Future;
use uuid::Uuid;
use std::collections::HashMap;
use crate::event::{IntoEvent, Event, EventData};
use failure::Error;
use crate::notification::{Sender, Listener, MemorySender, MemoryListener};
use std::sync::{Arc};
use slog::Logger;
use tokio::time::delay_for;
use prometheus::HistogramVec;
use prometheus::CounterVec;
use futures::stream::BoxStream;
use std::time::Duration;


lazy_static! {
    static ref BUILD_AGGREGATE_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "eventific_build_aggregate_time_seconds",
        "The time it takes to build an aggregate",
        &["aggregateid"]
    )
    .unwrap();
    static ref BUILD_AGGREGATE_ERROR_COUNTER: CounterVec = register_counter_vec!(
        "eventific_build_aggregate_error_count",
        "Number of errors while building an aggregate",
        &["aggregateid", "error"]
    )
    .unwrap();
    static ref AGGREGATE_UPDATES_RECEIVED_COUNTER: CounterVec = register_counter_vec!(
        "eventific_aggregate_updates_received_count",
        "Number of aggregate updates received",
        &["aggregateid"]
    )
    .unwrap();
}

pub struct Eventific<S: Send, D: EventData, St: Store<D> = MemoryStore<D>> {
    default_logger: Logger,
    store: St,
    state_builder: StateBuilder<S, D>,
    sender: Arc<dyn Sender>,
    listener: Arc<dyn Listener>,
    phantom_data: PhantomData<D>
}

impl<S: Send, D: EventData, St: Store<D>> Clone for Eventific<S, D, St> {
    fn clone(&self) -> Self {
        Self {
            default_logger: self.default_logger.clone(),
            store: self.store.clone(),
            state_builder: self.state_builder,
            sender: Arc::clone(&self.sender),
            listener: Arc::clone(&self.listener),
            phantom_data: PhantomData
        }
    }
}

impl<S: Default + Send, D: EventData + AsRef<str>, St: Store<D>> Eventific<S, D, St> {
    const MAX_ATTEMPTS: u64 = 10;

    pub fn builder() -> EventificBuilder<S, D, MemoryStore<D>, MemorySender, MemoryListener> {
        EventificBuilder::new()
    }

    pub(crate) fn create(logger: Logger, store: St, state_builder: StateBuilder<S, D>, sender: Arc<dyn Sender>, listener: Arc<dyn Listener>) -> Self {
        Self {
            default_logger: logger,
            store,
            state_builder,
            sender,
            listener,
            phantom_data: PhantomData
        }
    }

    pub async fn create_aggregate(&self, logger: Option<&Logger>, aggregate_id: Uuid, event_data: Vec<D>, metadata: Option<HashMap<String, String>>) -> Result<(), EventificError<D>> {
        let logger = self.extract_logger(&logger);
        let events = event_data.into_event(aggregate_id, 0, metadata);
        let sender = Arc::clone(&self.sender);
        let event_count = events.len();

        Self::print_event_info(&logger, &events);

        self.store.save_events(&logger, events).await.map_err(EventificError::StoreError)?;

        info!(logger, "Created new aggregate and inserted {} new events", event_count; "aggregate_id" => aggregate_id.to_string());

        sender.send(&logger, aggregate_id)
            .await
            .map_err(EventificError::SendNotificationError)?;

        Ok(())
    }

    fn extract_logger<'a>(&'a self, logger: &Option<&'a Logger>) -> &'a Logger {
        logger.unwrap_or(&self.default_logger)
    }

    fn print_event_info(logger: &Logger, event_data: &Vec<Event<D>>)
    {
        for event in event_data {
            info!(logger, "Preparing event of type {} with id {}", event.payload.as_ref(), event.event_id; "aggregate_id" => event.aggregate_id.to_string());
        }
    }

    pub async fn aggregate(&self, logger: &Option<&Logger>, aggregate_id: Uuid) -> Result<Aggregate<S>, EventificError<D>> {
        let logger = self.extract_logger(&logger);
        let timer = BUILD_AGGREGATE_HISTOGRAM.with_label_values(&[&aggregate_id.to_string()]).start_timer();

        let events = self.store.events(&logger, aggregate_id).await
            .map_err(EventificError::StoreError)
            .map_err(move |err| {
                BUILD_AGGREGATE_ERROR_COUNTER.with_label_values(&[&aggregate_id.to_string(), &err.to_string()]).inc();
                err
            })?;

        let aggregate = Aggregate::from_events(&logger, self.state_builder, events)
            .await
            .map_err(move |err| {
                BUILD_AGGREGATE_ERROR_COUNTER.with_label_values(&[&aggregate_id.to_string(), &err.to_string()]).inc();
                err
            })?;

        timer.observe_duration();

        Ok(aggregate)
    }

    pub async fn add_events_to_aggregate<
        F: Fn(&Aggregate<S>) -> FF,
        FF: Future<Output = Result<Vec<D>, Error>>
    >(&self, logger: Option<&Logger>, aggregate_id: Uuid, _metadata: Option<HashMap<String, String>>, callback: F) -> Result<(), EventificError<D>> {
        let logger = self.extract_logger(&logger);
        let sender = Arc::clone(&self.sender);

        // We run this loop until we are a able to persist the events, or until we give up
        let mut attempts = 0;
        loop {
            let aggregate = {
                let timer = BUILD_AGGREGATE_HISTOGRAM.with_label_values(&[&aggregate_id.to_string()]).start_timer();

                let events = self.store.events(&logger, aggregate_id)
                    .await
                    .map_err(EventificError::StoreError)
                    .map_err(move |err| {
                        BUILD_AGGREGATE_ERROR_COUNTER.with_label_values(&[&aggregate_id.to_string(), &err.to_string()]).inc();
                        err
                    })?; // If we cant access the store we fail right away

                timer.observe_duration();

                let res = Aggregate::from_events(&logger, self.state_builder, events)
                    .await
                    .map_err(move |err| {
                        BUILD_AGGREGATE_ERROR_COUNTER.with_label_values(&[&aggregate_id.to_string(), &err.to_string()]).inc();
                        err
                    });

                res
            }?;

            let next_version = (aggregate.version + 1) as u32;

            let raw_events = callback(&aggregate)
                .into_future()
                .map_err(EventificError::ValidationError)
                .await?; // if validation fails, we exit

            let events = raw_events.into_event(aggregate.aggregate_id, next_version, None);
            let event_count = events.len();
            Self::print_event_info(&logger, &events);

            match self.store.save_events(&logger, events).await {
                Ok(_) => {
                    info!(&logger, "Inserted {} new events", event_count; "aggregate_id" => aggregate.aggregate_id.to_string());
                    sender.send(&logger, aggregate_id)
                        .await
                        .map_err(EventificError::SendNotificationError)?;
                    return Ok(())
                },
                Err(err) => {
                    if let StoreError::EventAlreadyExists(_) = err {
                        if attempts < Self::MAX_ATTEMPTS {
                            attempts += 1;
                            delay_for(Duration::from_secs(1)).await;
                            continue;
                        } else {
                            return Err(EventificError::StoreError(err))
                        }
                    } else {
                        return Err(EventificError::StoreError(err))
                    }
                }
            }

        }
    }

    pub async fn total_events(&self, logger: &Option<&Logger>) -> Result<u64, EventificError<D>> {
        let logger = self.extract_logger(&logger);
        self.store.total_events(&logger)
            .await
            .map_err(EventificError::StoreError)
    }

    pub async fn total_events_for_aggregate(&self, logger: &Option<&Logger>, aggregate_id: Uuid) -> Result<u64, EventificError<D>> {
        let logger = self.extract_logger(&logger);
        self.store.total_events_for_aggregate(&logger, aggregate_id)
            .await
            .map_err(EventificError::StoreError)
    }

    pub async fn total_aggregates(&self, logger: &Option<&Logger>) -> Result<u64, EventificError<D>> {
        let logger = self.extract_logger(&logger);
        self.store.total_aggregates(&logger)
            .await
            .map_err(EventificError::StoreError)
    }

    pub async fn all_aggregates<'a>(&'a self, logger: &Option<&'a Logger>) -> Result<BoxStream<'a, Result<Aggregate<S>, EventificError<D>>>, EventificError<D>> {
        let logger = self.extract_logger(&logger);
        let ids = self.store.aggregate_ids(&logger)
            .await
            .map_err(EventificError::StoreError)?;

        let aggregate_stream = ids
            .map_err(EventificError::StoreError)
            .and_then(move |id| {
                async move {
                    self.aggregate(&Some(&logger.clone()), id).await
                }
            });

        let boxed_stream: BoxStream<_> = aggregate_stream.boxed();

        Ok(boxed_stream)
    }

    pub async fn updated_aggregates<'a>(&'a self, logger: &Option<&'a Logger>) -> Result<BoxStream<'a, Result<Aggregate<S>, EventificError<D>>>, EventificError<D>> {
        let logger = self.extract_logger(&logger);

        let event_stream = self.listener.listen(&logger)
            .await
            .map_err(EventificError::ListenNotificationError)?;

        let aggregate_stream = event_stream
            .map_err(EventificError::ListenNotificationError)
            .and_then(move |id| {
                let logger = logger.clone();
                async move {
                    AGGREGATE_UPDATES_RECEIVED_COUNTER.with_label_values(&[&id.to_string()]).inc();

                    match self.aggregate(&Some(&logger), id).await {
                        Ok(aggregate) => {Ok(Some(aggregate))},
                        Err(err) => {
                            warn!(logger, "Error occurred while processing aggregate, the error was: {}", err);
                            Ok(None)
                        },
                    }
                }
            })
            .try_filter_map(|x| async { Ok(x) });

        let boxed_stream: BoxStream<_> = aggregate_stream.boxed();

        Ok(boxed_stream)
    }
}

#[cfg(test)]
mod test {
    use crate::Eventific;
    use crate::store::MemoryStore;
    use crate::aggregate::noop_builder;
    use crate::notification::create_memory_notification_pair;
    use std::sync::Arc;
    use slog::Logger;

    #[derive(Default)]
    struct FakeState;

    #[derive(Debug, Clone, strum_macros::EnumIter, strum_macros::AsRefStr)]
    enum FakeEvent {
        Test
    }

    #[test]
    fn create_should_run_without_errors() {
        let logger = Logger::root(
            slog::Discard,
            o!(),
        );
        let (sender, listener) = create_memory_notification_pair();
        let _result: Eventific<FakeState, FakeEvent, MemoryStore<FakeEvent>> = Eventific::create(logger, MemoryStore::new(), noop_builder, Arc::new(sender), Arc::new(listener));
    }
}
