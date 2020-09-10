use crate::store::{Store, StoreError};
use crate::EventificBuilder;
use crate::aggregate::{StateBuilder, Aggregate};
use crate::eventific::EventificError;
use futures::{TryFutureExt, TryStreamExt, StreamExt};
use std::future::Future;
use uuid::Uuid;
use crate::event::{IntoEvent, Event, EventData};
use failure::Error;
use slog::Logger;
use tokio::time::delay_for;
use futures::stream::BoxStream;
use std::time::Duration;
use crate::component::Component;
use futures::future::try_join_all;
use tokio::sync::broadcast;
use std::fmt::Debug;

/// Eventific, this is the main service used to interface with the event store
pub struct Eventific<S: Send, D: EventData, St: Store<D, M>, M: 'static + Send + Sync + Debug = ()> {
    default_logger: Logger,
    store: St,
    state_builder: StateBuilder<S, D, M>,
    event_published_sender: broadcast::Sender<Uuid>,
    event_received_sender: broadcast::Sender<Uuid>,
}

impl<S: Send, D: EventData, St: Store<D, M>, M: 'static + Send + Sync + Debug> Clone for Eventific<S, D, St, M> {
    fn clone(&self) -> Self {
        Self {
            default_logger: self.default_logger.clone(),
            store: self.store.clone(),
            state_builder: self.state_builder,
            event_published_sender: self.event_published_sender.clone(),
            event_received_sender: self.event_received_sender.clone(),
        }
    }
}

impl<S: 'static + Default + Send, D: EventData + AsRef<str>, St: Store<D, M>, M: 'static + Send + Sync + Debug + Clone> Eventific<S, D, St, M> {
    const MAX_ATTEMPTS: u64 = 10;

    pub fn builder() -> EventificBuilder<S, D, St, M> {
        EventificBuilder::new()
    }

    pub(crate) async fn create(
        logger: Logger,
        store: St,
        state_builder: StateBuilder<S, D, M>,
        components: Vec<Box<dyn Component<S, D, St, M>>>,
    ) -> Result<Self, EventificError<D, M>> {
        info!(logger, "Starting Eventific");

        info!(logger, "Available events are:");
        info!(logger, "");
        for event in D::iter() {
            info!(logger, "{}", event.as_ref());
        }
        info!(logger, "");

        info!(logger, "🤩  All setup and ready");

        let (event_published_sender, _) = broadcast::channel(1024);
        let (event_received_sender, _) = broadcast::channel(1024);
        let eventific = Self {
            default_logger: logger.clone(),
            store,
            state_builder,
            event_published_sender,
            event_received_sender,
        };

        try_join_all(components.into_iter().map(|mut comp| {
            let logger = logger.clone();
            let eventific = eventific.clone();
            async move {
                comp.init(logger.clone(), eventific.clone()).await
                    .map_err(|err| EventificError::ComponentInitError(comp.component_name(), err))?;
                Result::<(), EventificError<D, M>>::Ok(())
            }
        })).await?;

        Ok(eventific)
    }

    pub async fn create_aggregate(&self, logger: Option<&Logger>, aggregate_id: Uuid, event_data: Vec<D>, metadata: Option<M>) -> Result<(), EventificError<D, M>> {
        let logger = self.extract_logger(&logger);
        let events = event_data.into_event(aggregate_id, 0, metadata);
        let event_count = events.len();

        Self::print_event_info(&logger, &events);

        self.store.save_events(&logger, events).await.map_err(EventificError::StoreError)?;

        info!(logger, "Created new aggregate and inserted {} new events", event_count; "aggregate_id" => aggregate_id.to_string());

        self.event_published_sender.send(aggregate_id)
            .map_err(|err| EventificError::Unknown(format_err!("{:?}", err)))?;

        Ok(())
    }

    fn extract_logger<'a>(&'a self, logger: &Option<&'a Logger>) -> &'a Logger {
        logger.unwrap_or(&self.default_logger)
    }

    fn print_event_info(logger: &Logger, event_data: &Vec<Event<D, M>>)
    {
        for event in event_data {
            info!(logger, "Preparing event of type {} with id {}", event.payload.as_ref(), event.event_id; "aggregate_id" => event.aggregate_id.to_string());
        }
    }

    fn send_published_notification(&self, aggregate_id: Uuid) -> Result<(), EventificError<D, M>> {
        self.event_published_sender.send(aggregate_id)
            .map_err(|err| EventificError::Unknown(format_err!("{:?}", err)))?;
        Ok(())
    }

    pub async fn aggregate(&self, logger: &Option<&Logger>, aggregate_id: Uuid) -> Result<Aggregate<S>, EventificError<D, M>> {
        let logger = self.extract_logger(&logger);
        let events = self.store.events(&logger, aggregate_id).await
            .map_err(EventificError::StoreError)?;
        let aggregate = Aggregate::from_events(&logger, self.state_builder, events)
            .await?;
        Ok(aggregate)
    }

    pub async fn add_events_to_aggregate<
        F: Fn(&Aggregate<S>) -> FF,
        FF: Future<Output = Result<Vec<D>, Error>>
    >(&self, logger: Option<&Logger>, aggregate_id: Uuid, metadata: Option<M>, callback: F) -> Result<(), EventificError<D, M>> {
        let logger = self.extract_logger(&logger);

        // We run this loop until we are a able to persist the events, or until we give up
        let mut attempts = 0;
        loop {
            let aggregate = {
                let events = self.store.events(&logger, aggregate_id)
                    .await
                    .map_err(EventificError::StoreError)?; // If we cant access the store we fail right away
                let res = Aggregate::from_events(&logger, self.state_builder, events)
                    .await;
                res
            }?;

            let next_version = (aggregate.version() + 1) as u32;

            let raw_events = callback(&aggregate)
                .into_future()
                .map_err(EventificError::ValidationError)
                .await?; // if validation fails, we exit

            let events = raw_events.into_event(aggregate.id(), next_version, metadata.clone());
            let event_count = events.len();
            Self::print_event_info(&logger, &events);

            match self.store.save_events(&logger, events).await {
                Ok(_) => {
                    info!(&logger, "Inserted {} new events", event_count; "aggregate_id" => aggregate.id().to_string());
                    self.send_published_notification(aggregate_id)?;
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

    pub async fn total_events(&self, logger: &Option<&Logger>) -> Result<u64, EventificError<D, M>> {
        let logger = self.extract_logger(&logger);
        self.store.total_events(&logger)
            .await
            .map_err(EventificError::StoreError)
    }

    pub async fn total_events_for_aggregate(&self, logger: &Option<&Logger>, aggregate_id: Uuid) -> Result<u64, EventificError<D, M>> {
        let logger = self.extract_logger(&logger);
        self.store.total_events_for_aggregate(&logger, aggregate_id)
            .await
            .map_err(EventificError::StoreError)
    }

    pub async fn total_aggregates(&self, logger: &Option<&Logger>) -> Result<u64, EventificError<D, M>> {
        let logger = self.extract_logger(&logger);
        self.store.total_aggregates(&logger)
            .await
            .map_err(EventificError::StoreError)
    }

    pub async fn all_aggregates<'a>(&'a self, logger: &Option<&'a Logger>) -> Result<BoxStream<'a, Result<Aggregate<S>, EventificError<D, M>>>, EventificError<D, M>> {
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

    pub async fn updated_aggregates<'a>(&'a self, logger: &Option<&'a Logger>) -> Result<BoxStream<'a, Result<Aggregate<S>, EventificError<D, M>>>, EventificError<D, M>> {
        let logger = self.extract_logger(&logger);
        let listener = self.event_received_sender.subscribe();

        let aggregate_stream = listener.into_stream()
            .map_err(|err| EventificError::Unknown(format_err!("{}", err)))
            .and_then(move |id| {
                let logger = logger.clone();
                async move {
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
    use slog::Logger;
    use crate::event::EventData;

    #[derive(Default)]
    struct FakeState;

    #[derive(Debug, Clone, strum_macros::EnumIter, strum_macros::AsRefStr)]
    enum FakeEvent {
        Test
    }

    impl EventData for FakeEvent {}

    #[tokio::test]
    async fn create_should_run_without_errors() {
        let logger = Logger::root(
            slog::Discard,
            o!(),
        );
        let _result: Eventific<FakeState, FakeEvent, MemoryStore<FakeEvent, ()>> = Eventific::create(logger, MemoryStore::new(), |_|{}, vec![]).await.unwrap();
    }
}
