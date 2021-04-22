use crate::aggregate::{Aggregate, StateBuilder};
use crate::component::Component;
use crate::event::{Event, IntoEvent};
use crate::eventific::{AddEventsParams, CreateAggregateParams, EventificError};
use crate::store::{SaveEventsResult, Store, StoreContext};
use crate::EventificBuilder;
use futures::future::try_join_all;
use futures::stream::BoxStream;
use futures::{StreamExt, TryFutureExt, TryStreamExt};
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use strum::IntoEnumIterator;
use tokio::sync::broadcast;
use tokio::time::sleep;
use uuid::Uuid;
use itertools::join;
use tokio_stream::wrappers::{BroadcastStream};
use tokio_stream::wrappers::errors::{BroadcastStreamRecvError};
use std::fmt;

type EventificResult<T, St, D, M> = Result<T, EventificError<<St as Store>::Error, D, M>>;

/// Eventific, this is the main service used to interface with the event store
pub struct Eventific<
    St: Store<EventData = D, MetaData = M>,
    S: Send,
    D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator,
    M: 'static + Send + Sync + Debug = (),
> {
    store: Arc<St>,
    state_builder: StateBuilder<S, D, M>,
    event_published_sender: broadcast::Sender<Uuid>,
    event_received_sender: broadcast::Sender<Uuid>,
    service_name: String,
}

impl<
        St: Store<EventData = D, MetaData = M>,
        S: 'static + Send + Debug + Default,
        D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator + AsRef<str>,
        M: 'static + Send + Sync + Debug + Clone,
    > Clone for Eventific<St, S, D, M>
{

    #[tracing::instrument]
    fn clone(&self) -> Self {
        Self {
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
        S: 'static + Default + Send + Debug,
        D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator + AsRef<str>,
        M: 'static + Send + Sync + Debug + Clone,
    > Eventific<St, S, D, M>
{
    const MAX_ATTEMPTS: u64 = 10;

    /// Returns a new eventific builder
    #[tracing::instrument]
    pub fn builder() -> EventificBuilder<St, S, D, M> {
        EventificBuilder::new()
    }

    #[tracing::instrument(skip(state_builder))]
    pub async fn new(
        mut store: St,
        service_name: &str,
        state_builder: StateBuilder<S, D, M>,
        components: Vec<Box<dyn Component<St, S, D, M>>>,
    ) -> Result<Self, EventificError<St::Error, D, M>> {
        info!("Starting Eventific");

        let events_str = join(D::iter().map(|i| format!("{:#?}", i)), ",");

        info!("Available events are: {}", events_str);

        info!("🤩  All setup and ready");

        store
            .init(StoreContext {
                service_name: service_name.to_string(),
            })
            .await
            .map_err(EventificError::StoreInitError)?;

        let (event_published_sender, _) = broadcast::channel(1024);
        let (event_received_sender, _) = broadcast::channel(1024);
        let eventific = Self {
            store: Arc::new(store),
            state_builder,
            event_published_sender,
            event_received_sender,
            service_name: service_name.to_string(),
        };

        try_join_all(components.into_iter().map(|mut comp| {
            let eventific = eventific.clone();
            async move {
                comp.init(eventific.clone())
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
    ///         metadata: None
    ///     }
    /// ).await?;
    /// #
    /// #     Ok(())
    /// # }
    ///
    /// ```
    #[tracing::instrument]
    pub async fn create_aggregate(
        &self,
        params: CreateAggregateParams<D, M>,
    ) -> Result<(), EventificError<St::Error, D, M>> {
        let events = params
            .events
            .into_event(params.aggregate_id, 0, params.metadata);
        let event_count = events.len();

        Self::print_event_info(&events);

        self.store
            .save_events(self.create_store_context(), &events)
            .await
            .map_err(|e| EventificError::StoreError(e))?;

        info!("Created new aggregate '{}' and inserted {} new events", params.aggregate_id.to_string(), event_count);

        self.send_published_notification(params.aggregate_id);

        Ok(())
    }

    fn print_event_info(event_data: &Vec<Event<D, M>>) {
        for event in event_data {
            info!("Preparing event of type {} with id {} for aggregate '{}'", event.payload.as_ref(), event.event_id, event.aggregate_id.to_string());
        }
    }

    /// Broadcasts to components that new events has been published to an aggregate
    ///
    /// # Arguments
    /// * aggregate_id - The id of the aggregate that was updated
    #[tracing::instrument]
    fn send_published_notification(&self, aggregate_id: Uuid) -> () {
        if let Err(e) = self.event_published_sender.send(aggregate_id) {
            debug!("The seems to be no one listening for event stored event, returned message was '{:#?}'", e)
        }
    }

    /// Retrieves a aggregate from the store
    ///
    /// # Arguments
    /// * aggregate_id - The id of the aggregate to retrieve
    #[tracing::instrument]
    pub async fn aggregate(
        &self,
        aggregate_id: Uuid,
    ) -> Result<Aggregate<S>, EventificError<St::Error, D, M>> {
        let events = self
            .store
            .events(self.create_store_context(), aggregate_id)
            .await
            .map_err(EventificError::StoreError)?;
        let aggregate = Aggregate::from_events(
            self.state_builder,
            events.map_err(EventificError::StoreError),
        )
        .await?;
        Ok(aggregate)
    }

    /// Adds events to an existing aggregate
    #[tracing::instrument(skip(callback))]
    pub async fn add_events<
        F: Fn(&Aggregate<S>) -> FF,
        FF: Future<Output = Result<Vec<D>, E>>,
        E: 'static + std::error::Error + Send + Sync,
    >(
        &self,
        params: AddEventsParams<M>,
        callback: F,
    ) -> EventificResult<(), St, D, M> {

        // We run this loop until we are a able to persist the events, or until we give up
        let mut attempts = 0;
        loop {
            let aggregate = {
                let events = self
                    .store
                    .events(
                        self.create_store_context(),
                        params.aggregate_id,
                    )
                    .await
                    .map_err(EventificError::StoreError)?; // If we cant access the store we fail right away
                let res = Aggregate::from_events(
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
            Self::print_event_info(&events);

            let res: SaveEventsResult = self
                .store
                .save_events(self.create_store_context(), &events)
                .await
                .map_err(EventificError::StoreError)?;

            match res {
                SaveEventsResult::Success => {
                    info!("Inserted {} new events for aggregate '{}'", event_count, aggregate.id().to_string());
                    self.send_published_notification(params.aggregate_id);
                    return Ok(());
                }
                SaveEventsResult::AlreadyExists => {
                    if attempts < Self::MAX_ATTEMPTS {
                        attempts += 1;
                        sleep(Duration::from_secs(1)).await;
                        continue;
                    } else {
                        return Err(EventificError::InsertFailed(Self::MAX_ATTEMPTS, events));
                    }
                }
            }
        }
    }

    /// Returns the total amount of events in the event store
    #[tracing::instrument]
    pub async fn total_events(
        &self,
    ) -> Result<u64, EventificError<St::Error, D, M>> {
        self.store
            .total_events(self.create_store_context())
            .await
            .map_err(EventificError::StoreError)
    }

    /// Returns all events for a particular aggregate
    ///
    /// # Arguments
    /// * aggregate_id - The id of the aggregate to retrieve events for
    #[tracing::instrument]
    pub async fn total_events_for_aggregate(
        &self,
        aggregate_id: Uuid,
    ) -> Result<u64, EventificError<St::Error, D, M>> {
        self.store
            .total_events_for_aggregate(self.create_store_context(), aggregate_id)
            .await
            .map_err(EventificError::StoreError)
    }

    /// Returns the total amount of aggregates
    #[tracing::instrument]
    pub async fn total_aggregates(
        &self,
    ) -> Result<u64, EventificError<St::Error, D, M>> {
        self.store
            .total_aggregates(self.create_store_context())
            .await
            .map_err(EventificError::StoreError)
    }

    /// Creates a stream of all aggregates within the store
    #[tracing::instrument]
    pub async fn all_aggregates<'a>(
        &'a self,
    ) -> Result<
        BoxStream<'a, Result<Aggregate<S>, EventificError<St::Error, D, M>>>,
        EventificError<St::Error, D, M>,
    > {
        let ids = self
            .store
            .aggregate_ids(self.create_store_context())
            .await
            .map_err(EventificError::StoreError)?;

        let aggregate_stream = ids
            .map_err(EventificError::StoreError)
            .and_then(move |id| async move { self.aggregate(id).await });

        let boxed_stream: BoxStream<_> = aggregate_stream.boxed();

        Ok(boxed_stream)
    }

    /// Creates a stream that listens for all new or updated aggregates
    #[tracing::instrument]
    pub async fn updated_aggregates<'a>(
        &'a self,
    ) -> Result<
        BoxStream<'a, Result<Aggregate<S>, EventificError<St::Error, D, M>>>,
        EventificError<St::Error, D, M>,
    > {
        let listener = self.event_received_sender.subscribe();

        let aggregate_stream = BroadcastStream::new(listener)
            .filter(move |id_result| {
                if let Err(err) = id_result {
                    if let BroadcastStreamRecvError::Lagged(lagged_num) = err {
                        error!("The updated aggregates subscription can't keep up with all the new inserted ones, number off lagged inserts '{}'", lagged_num);
                    }

                    futures::future::ready(false)
                } else {
                    futures::future::ready(true)
                }
            })
            .map_err(|_e| unreachable!())
            .and_then(move |id| {
                async move {
                    match self.aggregate(id).await {
                        Ok(aggregate) => Ok(Some(aggregate)),
                        Err(err) => {
                            warn!(
                                "Error occurred while processing aggregate, the error was: {:#?}", err
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

    fn create_store_context(&self) -> StoreContext {
        StoreContext {
            service_name: self.service_name.to_string(),
        }
    }
}

impl<St: Store<EventData = D, MetaData = M>,
    S: 'static + Default + Send,
    D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator + AsRef<str>,
    M: 'static + Send + Sync + Debug + Clone> Debug for Eventific<St, S, D, M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Service Name: {}", self.service_name)?;
        write!(f, "Store: {:#?}", self.store)
    }
}

#[cfg(test)]
mod test {
    use crate::store::MemoryStore;
    use crate::Eventific;

    #[derive(Default, Debug)]
    struct FakeState;

    #[derive(Debug, Clone, strum_macros::EnumIter, strum_macros::AsRefStr)]
    enum FakeEvent {
        Test,
    }

    #[tokio::test]
    async fn create_should_run_without_errors() {
        let _result: Eventific<MemoryStore<FakeEvent, ()>, FakeState, FakeEvent> =
            Eventific::new(MemoryStore::new(), "TEST", |_| {}, vec![])
                .await
                .unwrap();
    }
}
