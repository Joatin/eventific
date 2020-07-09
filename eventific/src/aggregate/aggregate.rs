use std::fmt::Debug;
use crate::aggregate::StateBuilder;
use crate::event::{Event, EventData};
use uuid::Uuid;
use crate::eventific::EventificError;
use chrono::{DateTime, Utc};
use slog::Logger;
use futures::{TryStreamExt};
use crate::store::StoreError;
use futures::stream::BoxStream;


/// An aggregate representation. This will contain all available information about the aggregate, including its state
#[derive(Debug, Clone)]
pub struct Aggregate<S: Send> {
    /// The id of this aggregate
    pub aggregate_id: Uuid,
    /// The date this aggregate was first created
    pub created_date: DateTime<Utc>,
    /// The last time this aggregate was updated
    pub last_updated_date: DateTime<Utc>,
    /// The current version of this aggregate, this is the same as the event id of the latest event added to this aggregate
    pub version: i32,
    /// The state of this aggregate
    pub state: S
}

impl<S: Default + Send> Aggregate<S> {
    pub(crate) async fn from_events<D: EventData>(logger: &Logger, state_builder: StateBuilder<S, D>, events: BoxStream<'_, Result<Event<D>, StoreError<D>>>) -> Result<Self, EventificError<D>> {

        let initial_aggregate = Self {
            aggregate_id: Uuid::nil(),
            created_date: Utc::now(),
            last_updated_date: Utc::now(),
            version: -1,
            state: S::default()
        };

        let aggregate = events
            .map_err(EventificError::StoreError)
            .try_fold(initial_aggregate, |mut aggregate, event| async {
                if (event.event_id as i32) != (aggregate.version + 1) {
                    return Err(EventificError::InconsistentEventChain(event))
                }
                debug!(logger, "Building aggregate with event: \n{:#?}", event);
                aggregate.aggregate_id = event.aggregate_id;
                aggregate.version += 1;
                state_builder(&mut aggregate.state, &event);
                Ok(aggregate)
            }).await?;

        info!(logger, "Done building aggregate '{}' with {} events", &aggregate.aggregate_id, aggregate.version + 1);

        Ok(aggregate)
    }
}

#[cfg(test)]
mod test {
    use crate::aggregate::{Aggregate, noop_builder};
    use uuid::Uuid;
    use crate::event::{IntoEvent, Event};
    use crate::eventific::EventificError;
    use slog::Logger;

    #[derive(Default, Debug)]
    struct TestState {
        text: String
    }

    #[derive(Debug, Clone)]
    enum TestEventData {
        Test
    }

    fn setup_events() -> (Uuid, Vec<Event<TestEventData>>) {
        let id = Uuid::parse_str("4355f3e6-be3e-4a91-a8a8-b967db878f5b").unwrap();

        let event_data = vec![
            TestEventData::Test,
            TestEventData::Test,
            TestEventData::Test,
        ];

        let events = event_data.into_event(id, 0, None);

        (id, events)
    }

    #[test]
    fn from_events_should_set_aggregate_id() {
        let logger = Logger::root(
            slog::Discard,
            o!(),
        );
        let (id, events) = setup_events();
        let aggregate: Aggregate<TestState> = Aggregate::from_events(&logger, noop_builder, &events).unwrap();
        assert_eq!(aggregate.aggregate_id, id);
    }

    #[test]
    fn from_events_should_set_correct_version() {
        let logger = Logger::root(
            slog::Discard,
            o!(),
        );
        let (id, events) = setup_events();
        let aggregate: Aggregate<TestState> = Aggregate::from_events(&logger, noop_builder, &events).unwrap();
        assert_eq!(aggregate.version, 2);
    }

    #[test]
    fn from_events_should_set_correct_created_date() {
        let logger = Logger::root(
            slog::Discard,
            o!(),
        );
        let (id, events) = setup_events();
        let aggregate: Aggregate<TestState> = Aggregate::from_events(&logger, noop_builder, &events).unwrap();
        assert_eq!(aggregate.created_date, events[0].created_date);
    }

    #[test]
    fn from_events_should_set_correct_last_updated_date() {
        let logger = Logger::root(
            slog::Discard,
            o!(),
        );
        let (id, events) = setup_events();
        let aggregate: Aggregate<TestState> = Aggregate::from_events(&logger, noop_builder, &events).unwrap();
        assert_eq!(aggregate.last_updated_date, events[2].created_date);
    }

    #[test]
    fn from_events_should_set_correct_state() {
        let logger = Logger::root(
            slog::Discard,
            o!(),
        );
        let (id, events) = setup_events();
        fn state_builder(state: TestState, event: &Event<TestEventData>) -> TestState {
            TestState {
                text: "Hello World".to_owned()
            }
        }
        let aggregate: Aggregate<TestState> = Aggregate::from_events(&logger, state_builder, &events).unwrap();
        assert_eq!(aggregate.state.text, "Hello World");
    }

    #[test]
    fn from_events_should_return_error_if_events_are_empty() {
        let logger = Logger::root(
            slog::Discard,
            o!(),
        );
        let error = Aggregate::<TestState>::from_events::<TestEventData>(&logger, noop_builder, &Vec::new()).unwrap_err();
        if let EventificError::Unknown(_) = error {
            // Yay, this is correct
        } else {
            panic!("Wrong error type")
        }
    }

}
