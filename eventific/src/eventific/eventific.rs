use crate::aggregate::{Aggregate, StateBuilder};
use crate::component::Component;
use crate::event::{Event, IntoEvent};
use crate::eventific::{AddEventsParams, CreateAggregateParams, EventificError};
use crate::store::{SaveEventsResult, Store, StoreContext};
use crate::EventificBuilder;
use futures::future::try_join_all;
use futures::stream::BoxStream;
use futures::{StreamExt, TryFutureExt, TryStreamExt};
use slog::Logger;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use strum::IntoEnumIterator;
use tokio::sync::broadcast;
use tokio::time::delay_for;
use uuid::Uuid;

type EventificResult<T, St, D, M> = Result<T, EventificError<<St as Store>::Error, D, M>>;

/// Eventific, this is the main service used to interface with the event store
pub struct Eventific<
    St: Store<EventData = D, MetaData = M>,
    S: Send,
    D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator,
    M: 'static + Send + Sync + Debug = (),
> {
    default_logger: Logger,
    store: Arc<St>,
    state_builder: StateBuilder<S, D, M>,
    event_published_sender: broadcast::Sender<Uuid>,
    event_received_sender: broadcast::Sender<Uuid>,
    service_name: String,
}

impl<
        St: Store<EventData = D, MetaData = M>,
        S: Send,
        D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator,
        M: 'static + Send + Sync + Debug,
    > Clone for Eventific<St, S, D, M>
{
    fn clone(&self) -> Self {
        Self {
            default_logger: self.default_logger.clone(),
            store: Arc::clone(&self.store),
            state_builder: self.state_builder,
            event_published_sender: self.event_published_sender.clone(),
            event_received_sender: self.event_received_sender.clone(),
            service_name: self.service_name.to_string(),
        }
    }
}

impl<
        St: Store<EventData = D, MetaData = M>,
        S: 'static + Default + Send,
        D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator + AsRef<str>,
        M: 'static + Send + Sync + Debug + Clone,
    > Eventific<St, S, D, M>
{
    const MAX_ATTEMPTS: u64 = 10;

    /// Returns a new eventific builder
    pub fn builder() -> EventificBuilder<St, S, D, M> {
        EventificBuilder::new()
    }

    pub async fn new(
        logger: Logger,
        mut store: St,
        service_name: &str,
        state_builder: StateBuilder<S, D, M>,
        components: Vec<Box<dyn Component<St, S, D, M>>>,
    ) -> Result<Self, EventificError<St::Error, D, M>> {
        info!(logger, "Starting Eventific");

        info!(logger, "Available events are:");
        info!(logger, "");
        for event in D::iter() {
            info!(logger, "{}", event.as_ref());
        }
        info!(logger, "");

        info!(logger, "🤩  All setup and ready");

        store
            .init(StoreContext {
                logger: logger.clone(),
                service_name: service_name.to_string(),
            })
            .await
            .map_err(EventificError::StoreInitError)?;

        let (event_published_sender, _) = broadcast::channel(1024);
        let (event_received_sender, _) = broadcast::channel(1024);
        let eventific = Self {
            default_logger: logger.clone(),
            store: Arc::new(store),
            state_builder,
            event_published_sender,
            event_received_sender,
            service_name: service_name.to_string(),
        };

        try_join_all(components.into_iter().map(|mut comp| {
            let logger = logger.clone();
            let eventific = eventific.clone();
            async move {
                comp.init(logger.clone(), eventific.clone())
                    .await
                    .map_err(|err| {
                        EventificError::ComponentInitError(comp.component_name().to_string(), err)
                    })?;
                Result::<(), EventificError<St::Error, D, M>>::Ok(())
            }
        }))
        .await?;

        Ok(eventific)
    }

    /// Creates a new aggregate within the event store
    ///
    /// # Examples
    ///
    /// ```
    /// # use uuid::Uuid;
    /// # use eventific::EventificBuilder;
    /// # use eventific::store::MemoryStore;
    /// # use eventific::CreateAggregateParams;
    /// # use eventific::Eventific;
    /// #
    /// # #[derive(Debug, Clone, strum_macros::EnumIter, strum_macros::AsRefStr)]
    /// # enum EventData {
    /// #     Created
    /// # }
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let eventific: Eventific<MemoryStore<EventData, ()>, (), EventData, ()> = EventificBuilder::new().build("TEST", |_| {}, MemoryStore::new()).await?;
    /// #
    /// eventific.create_aggregate(
    ///     CreateAggregateParams {
    ///         aggregate_id: Uuid::nil(),
    ///         events: vec![
    ///             EventData::Created
    ///          ],
    ///         metadata: None,
    ///         logger: None
    ///     }
    /// ).await?;
    /// #
    /// #     Ok(())
    /// # }
    ///
    /// ```
    pub async fn create_aggregate(
        &self,
        params: CreateAggregateParams<D, M>,
    ) -> Result<(), EventificError<St::Error, D, M>> {
        let logger = params.logger.unwrap_or_else(|| self.default_logger.clone());
        let events = params
            .events
            .into_event(params.aggregate_id, 0, params.metadata);
        let event_count = events.len();

        Self::print_event_info(&logger, &events);

        self.store
            .save_events(self.create_store_context(logger.clone()), &events)
            .await
            .map_err(|e| EventificError::StoreError(e))?;

        info!(logger, "Created new aggregate and inserted {} new events", event_count; "aggregate_id" => params.aggregate_id.to_string());

        self.send_published_notification(params.aggregate_id);

        Ok(())
    }

    fn extract_logger<'a>(&'a self, logger: &Option<&'a Logger>) -> &'a Logger {
        logger.unwrap_or(&self.default_logger)
    }

    fn print_event_info(logger: &Logger, event_data: &Vec<Event<D, M>>) {
        for event in event_data {
            info!(logger, "Preparing event of type {} with id {}", event.payload.as_ref(), event.event_id; "aggregate_id" => event.aggregate_id.to_string());
        }
    }

    /// Broadcasts to components that new events has been published to an aggregate
    ///
    /// # Arguments
    /// * aggregate_id - The id of the aggregate that was uppdated
    fn send_published_notification(&self, aggregate_id: Uuid) -> () {
        if let Err(e) = self.event_published_sender.send(aggregate_id) {
            debug!(self.extract_logger(&None), "The seems to be no one listening for event stored event, returned message was '{:#?}'", e)
        }
    }

    pub async fn aggregate(
        &self,
        logger: &Option<&Logger>,
        aggregate_id: Uuid,
    ) -> Result<Aggregate<S>, EventificError<St::Error, D, M>> {
        let logger = self.extract_logger(&logger);
        let events = self
            .store
            .events(self.create_store_context(logger.clone()), aggregate_id)
            .await
            .map_err(EventificError::StoreError)?;
        let aggregate = Aggregate::from_events(
            &logger,
            self.state_builder,
            events.map_err(EventificError::StoreError),
        )
        .await?;
        Ok(aggregate)
    }

    /// Adds events to an existing aggregate
    pub async fn add_events<
        F: Fn(&Aggregate<S>) -> FF,
        FF: Future<Output = Result<Vec<D>, E>>,
        E: 'static + std::error::Error + Send + Sync,
    >(
        &self,
        params: AddEventsParams<M>,
        callback: F,
    ) -> EventificResult<(), St, D, M> {
        let logger = params.logger.unwrap_or_else(|| self.default_logger.clone());

        // We run this loop until we are a able to persist the events, or until we give up
        let mut attempts = 0;
        loop {
            let aggregate = {
                let events = self
                    .store
                    .events(
                        self.create_store_context(logger.clone()),
                        params.aggregate_id,
                    )
                    .await
                    .map_err(EventificError::StoreError)?; // If we cant access the store we fail right away
                let res = Aggregate::from_events(
                    &logger,
                    self.state_builder,
                    events.map_err(EventificError::StoreError),
                )
                .await;
                res
            }?;

            let next_version = (aggregate.version() + 1) as u32;

            let raw_events = callback(&aggregate)
                .into_future()
                .map_err(|err| EventificError::ValidationError(Box::new(err)))
                .await?; // if validation fails, we exit

            let events =
                raw_events.into_event(aggregate.id(), next_version, params.metadata.clone());
            let event_count = events.len();
            Self::print_event_info(&logger, &events);

            let res: SaveEventsResult = self
                .store
                .save_events(self.create_store_context(logger.clone()), &events)
                .await
                .map_err(EventificError::StoreError)?;

            match res {
                SaveEventsResult::Success => {
                    info!(&logger, "Inserted {} new events", event_count; "aggregate_id" => aggregate.id().to_string());
                    self.send_published_notification(params.aggregate_id);
                    return Ok(());
                }
                SaveEventsResult::AlreadyExists => {
                    if attempts < Self::MAX_ATTEMPTS {
                        attempts += 1;
                        delay_for(Duration::from_secs(1)).await;
                        continue;
                    } else {
                        return Err(EventificError::InsertFailed(Self::MAX_ATTEMPTS, events));
                    }
                }
            }
        }
    }

    pub async fn total_events(
        &self,
        logger: &Option<&Logger>,
    ) -> Result<u64, EventificError<St::Error, D, M>> {
        let logger = self.extract_logger(&logger);
        self.store
            .total_events(self.create_store_context(logger.clone()))
            .await
            .map_err(EventificError::StoreError)
    }

    pub async fn total_events_for_aggregate(
        &self,
        logger: &Option<&Logger>,
        aggregate_id: Uuid,
    ) -> Result<u64, EventificError<St::Error, D, M>> {
        let logger = self.extract_logger(&logger);
        self.store
            .total_events_for_aggregate(self.create_store_context(logger.clone()), aggregate_id)
            .await
            .map_err(EventificError::StoreError)
    }

    pub async fn total_aggregates(
        &self,
        logger: &Option<&Logger>,
    ) -> Result<u64, EventificError<St::Error, D, M>> {
        let logger = self.extract_logger(&logger);
        self.store
            .total_aggregates(self.create_store_context(logger.clone()))
            .await
            .map_err(EventificError::StoreError)
    }

    pub async fn all_aggregates<'a>(
        &'a self,
        logger: &Option<&'a Logger>,
    ) -> Result<
        BoxStream<'a, Result<Aggregate<S>, EventificError<St::Error, D, M>>>,
        EventificError<St::Error, D, M>,
    > {
        let logger = self.extract_logger(&logger);
        let ids = self
            .store
            .aggregate_ids(self.create_store_context(logger.clone()))
            .await
            .map_err(EventificError::StoreError)?;

        let aggregate_stream = ids
            .map_err(EventificError::StoreError)
            .and_then(move |id| async move { self.aggregate(&Some(&logger.clone()), id).await });

        let boxed_stream: BoxStream<_> = aggregate_stream.boxed();

        Ok(boxed_stream)
    }

    pub async fn updated_aggregates<'a>(
        &'a self,
        logger: &Option<&'a Logger>,
    ) -> Result<
        BoxStream<'a, Result<Aggregate<S>, EventificError<St::Error, D, M>>>,
        EventificError<St::Error, D, M>,
    > {
        let logger = self.extract_logger(&logger);
        let listener = self.event_received_sender.subscribe();

        let aggregate_stream = listener
            .into_stream()
            .map_err(|err| EventificError::Unknown(Box::new(err)))
            .and_then(move |id| {
                let logger = logger.clone();
                async move {
                    match self.aggregate(&Some(&logger), id).await {
                        Ok(aggregate) => Ok(Some(aggregate)),
                        Err(err) => {
                            warn!(
                                logger,
                                "Error occurred while processing aggregate, the error was: {}", err
                            );
                            Ok(None)
                        }
                    }
                }
            })
            .try_filter_map(|x| async { Ok(x) });

        let boxed_stream: BoxStream<_> = aggregate_stream.boxed();

        Ok(boxed_stream)
    }

    fn create_store_context(&self, logger: Logger) -> StoreContext {
        StoreContext {
            logger,
            service_name: self.service_name.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::store::MemoryStore;
    use crate::Eventific;
    use slog::Logger;

    #[derive(Default)]
    struct FakeState;

    #[derive(Debug, Clone, strum_macros::EnumIter, strum_macros::AsRefStr)]
    enum FakeEvent {
        Test,
    }

    #[tokio::test]
    async fn create_should_run_without_errors() {
        let logger = Logger::root(slog::Discard, o!());
        let _result: Eventific<MemoryStore<FakeEvent, ()>, FakeState, FakeEvent> =
            Eventific::new(logger, MemoryStore::new(), "TEST", |_| {}, vec![])
                .await
                .unwrap();
    }
}
