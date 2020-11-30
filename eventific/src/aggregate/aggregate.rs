use crate::aggregate::StateBuilder;
use crate::event::Event;
use crate::eventific::EventificError;
use chrono::{DateTime, Utc};
use futures::stream::BoxStream;
use futures::{Stream, TryStreamExt};
use slog::Logger;
use std::fmt::Debug;
use uuid::Uuid;

/// An aggregate representation. This will contain all available information about the aggregate, including its state
#[derive(Debug, Clone)]
pub struct Aggregate<S: Send> {
    aggregate_id: Uuid,
    created_date: DateTime<Utc>,
    last_updated_date: DateTime<Utc>,
    version: i32,
    state: S,
}

impl<S: Default + Send> Aggregate<S> {
    /// The id of this aggregate
    pub fn id(&self) -> Uuid {
        self.aggregate_id
    }

    /// The date this aggregate was first created
    pub fn created_date(&self) -> DateTime<Utc> {
        self.created_date
    }

    /// The last time this aggregate was updated
    pub fn last_updated_date(&self) -> DateTime<Utc> {
        self.last_updated_date
    }

    /// The current version of this aggregate, this is the same as the event id of the latest event added to this aggregate
    pub fn version(&self) -> i32 {
        self.version
    }

    /// The state of this aggregate
    pub fn state(&self) -> &S {
        &self.state
    }

    pub(crate) async fn from_events<
        StoreError: 'static + std::error::Error + Send + Sync,
        D: 'static + Debug + Clone + Send + Sync,
        M: 'static + Send + Sync + Debug,
        SS: Stream<Item = Result<Event<D, M>, EventificError<StoreError, D, M>>>,
    >(
        logger: &Logger,
        state_builder: StateBuilder<S, D, M>,
        events: SS,
    ) -> Result<Self, EventificError<StoreError, D, M>> {
        let initial_aggregate = Self::default();

        let aggregate = events
            .try_fold(initial_aggregate, |mut aggregate, event| async {
                if (event.event_id as i32) != (aggregate.version + 1) {
                    return Err(EventificError::InconsistentEventChain(event));
                }
                debug!(logger, "Building aggregate with event: \n{:#?}", event);
                aggregate.aggregate_id = event.aggregate_id;
                aggregate.version += 1;
                state_builder((&mut aggregate.state, &event));
                Ok(aggregate)
            })
            .await?;

        if aggregate.is_empty() {
            return Err(EventificError::BuildAggregateFromZeroEvents);
        }

        info!(
            logger,
            "Done building aggregate '{}' with {} events",
            &aggregate.aggregate_id,
            aggregate.version + 1
        );

        Ok(aggregate)
    }

    /// True if this aggregate has not been sourced from any events
    pub fn is_empty(&self) -> bool {
        self.version == -1
    }
}

impl<S: Send + Default> Default for Aggregate<S> {
    fn default() -> Self {
        Self {
            aggregate_id: Uuid::nil(),
            created_date: Utc::now(),
            last_updated_date: Utc::now(),
            version: -1,
            state: S::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::aggregate::Aggregate;
    use crate::event::{Event, IntoEvent};
    use crate::eventific::EventificError;
    use futures::stream::BoxStream;
    use futures::StreamExt;
    use slog::Logger;
    use uuid::Uuid;

    #[derive(Default, Debug)]
    struct TestState {
        text: String,
    }

    #[derive(Debug, Clone, strum_macros::EnumIter)]
    enum TestEventData {
        Test,
    }

    #[derive(Debug, thiserror::Error)]
    #[error("This is an error")]
    struct TestError;

    fn setup_events<'a>() -> (
        Uuid,
        BoxStream<
            'a,
            Result<Event<TestEventData, ()>, EventificError<TestError, TestEventData, ()>>,
        >,
    ) {
        let id = Uuid::parse_str("4355f3e6-be3e-4a91-a8a8-b967db878f5b").unwrap();

        let event_data = vec![
            TestEventData::Test,
            TestEventData::Test,
            TestEventData::Test,
        ];

        let events = futures::stream::iter(event_data.into_event(id, 0, None))
            .map(|i| Ok(i))
            .boxed();

        (id, events)
    }

    #[tokio::test]
    async fn from_events_should_set_aggregate_id() {
        let logger = Logger::root(slog::Discard, o!());
        let (id, events) = setup_events();
        let aggregate: Aggregate<TestState> = Aggregate::from_events(&logger, |_| {}, events)
            .await
            .unwrap();
        assert_eq!(aggregate.aggregate_id, id);
    }

    #[tokio::test]
    async fn from_events_should_set_correct_version() {
        let logger = Logger::root(slog::Discard, o!());
        let (_id, events) = setup_events();
        let aggregate: Aggregate<TestState> = Aggregate::from_events(&logger, |_| {}, events)
            .await
            .unwrap();
        assert_eq!(aggregate.version, 2);
    }

    // #[tokio::test]
    // async fn from_events_should_set_correct_created_date() {
    //     let logger = Logger::root(
    //         slog::Discard,
    //         o!(),
    //     );
    //     let (id, events) = setup_events();
    //     let aggregate: Aggregate<TestState> = Aggregate::from_events(&logger, |_| {}, events).await.unwrap();
    //     assert_eq!(aggregate.created_date, events[0].created_date);
    // }
    //
    // #[tokio::test]
    // async fn from_events_should_set_correct_last_updated_date() {
    //     let logger = Logger::root(
    //         slog::Discard,
    //         o!(),
    //     );
    //     let (id, events) = setup_events();
    //     let aggregate: Aggregate<TestState> = Aggregate::from_events(&logger, |_| {}, events).await.unwrap();
    //     assert_eq!(aggregate.last_updated_date, events[2].created_date);
    // }

    #[tokio::test]
    async fn from_events_should_set_correct_state() {
        let logger = Logger::root(slog::Discard, o!());
        let (_id, events) = setup_events();
        fn state_builder((state, _event): (&mut TestState, &Event<TestEventData, ()>)) -> () {
            state.text = "Hello World".to_owned()
        }
        let aggregate: Aggregate<TestState> =
            Aggregate::from_events(&logger, state_builder, events)
                .await
                .unwrap();
        assert_eq!(aggregate.state.text, "Hello World");
    }

    #[tokio::test]
    async fn from_events_should_return_error_if_events_are_empty() {
        let logger = Logger::root(slog::Discard, o!());
        fn state_builder((state, _event): (&mut TestState, &Event<TestEventData, ()>)) -> () {
            state.text = "Hello World".to_owned()
        }
        let error = Aggregate::from_events(
            &logger,
            state_builder,
            futures::stream::empty::<
                Result<Event<TestEventData, ()>, EventificError<TestError, TestEventData, ()>>,
            >(),
        )
        .await
        .unwrap_err();
        if let EventificError::BuildAggregateFromZeroEvents = error {
            // Yay, this is correct
        } else {
            panic!("Wrong error type")
        }
    }
}
