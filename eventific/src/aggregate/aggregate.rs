use std::fmt::{Display, Formatter, Debug};
use std::fmt;
use crate::aggregate::StateBuilder;
use crate::event::Event;
use uuid::Uuid;
use crate::eventific::EventificError;
use chrono::{DateTime, Utc};


/// An aggregate representation. This will contain all available information about the aggregate, including its state
#[derive(Debug, Clone)]
pub struct Aggregate<S> {
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

impl<S: Default> Aggregate<S> {
    pub(crate) fn from_events<D: 'static + Send + Sync + Debug + Clone>(state_builder: StateBuilder<S, D>, events: &[Event<D>]) -> Result<Self, EventificError<D>> {
        if events.is_empty() {
            return Err(EventificError::Unknown(format_err!("Can't build an aggregate from an empty set of events")))
        }

        let mut state = S::default();
        let mut version = -1;

        for event in events {
            if (event.event_id as i32) != (version + 1) {
                return Err(EventificError::InconsistentEventChain(events.to_vec()))
            }
            version += 1;
            state = state_builder(state, event);
        }

        Ok(Self {
            aggregate_id: events[0].aggregate_id,
            created_date: events[0].created_date.clone(),
            last_updated_date: events[events.len() - 1].created_date.clone(),
            version,
            state
        })
    }
}

#[cfg(test)]
mod test {
    use crate::aggregate::{Aggregate, noop_builder};
    use uuid::Uuid;
    use crate::event::{IntoEvent, Event};
    use crate::eventific::EventificError;

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
        let (id, events) = setup_events();
        let aggregate: Aggregate<TestState> = Aggregate::from_events(noop_builder, &events).unwrap();
        assert_eq!(aggregate.aggregate_id, id);
    }

    #[test]
    fn from_events_should_set_correct_version() {
        let (id, events) = setup_events();
        let aggregate: Aggregate<TestState> = Aggregate::from_events(noop_builder, &events).unwrap();
        assert_eq!(aggregate.version, 2);
    }

    #[test]
    fn from_events_should_set_correct_created_date() {
        let (id, events) = setup_events();
        let aggregate: Aggregate<TestState> = Aggregate::from_events(noop_builder, &events).unwrap();
        assert_eq!(aggregate.created_date, events[0].created_date);
    }

    #[test]
    fn from_events_should_set_correct_last_updated_date() {
        let (id, events) = setup_events();
        let aggregate: Aggregate<TestState> = Aggregate::from_events(noop_builder, &events).unwrap();
        assert_eq!(aggregate.last_updated_date, events[2].created_date);
    }

    #[test]
    fn from_events_should_set_correct_state() {
        let (id, events) = setup_events();
        fn state_builder(state: TestState, event: &Event<TestEventData>) -> TestState {
            TestState {
                text: "Hello World".to_owned()
            }
        }
        let aggregate: Aggregate<TestState> = Aggregate::from_events(state_builder, &events).unwrap();
        assert_eq!(aggregate.state.text, "Hello World");
    }

    #[test]
    fn from_events_should_return_error_if_events_are_empty() {
        let error = Aggregate::<TestState>::from_events::<TestEventData>(noop_builder, &Vec::new()).unwrap_err();
        if let EventificError::Unknown(_) = error {
            // Yay, this is correct
        } else {
            panic!("Wrong error type")
        }
    }

}
