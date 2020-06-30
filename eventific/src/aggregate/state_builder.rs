use crate::event::{Event, EventData};

/// Used to compute the state
///
/// The state builder should be a "pure" function. This means that it should have zero side effects and not depend on
/// any external resources, with the same input it should **always** produce the same output.
///
/// The state builder is where most of your business logic (not validation though) will reside. If you are going to test
/// anything in your app, this should be your highest priority
pub type StateBuilder<S, D> = fn(&mut S, &Event<D>);

pub(crate) fn noop_builder<S: Send, D: EventData>(_state: &mut S, _event: &Event<D>) {}


#[cfg(test)]
mod test {
    use crate::aggregate::noop_builder;
    use crate::event::Event;
    use uuid::Uuid;
    use std::collections::HashMap;
    use chrono::Utc;

    #[derive(Default)]
    struct TestState(String);

    enum TestEventData {
        Test
    }

    #[test]
    fn noop_builder_should_return_the_same_state() {
        let mut state = TestState("TEST".to_owned());

        let event = Event {
            aggregate_id: Uuid::nil(),
            event_id: 0,
            created_date: Utc::now(),
            metadata: HashMap::new(),
            payload: TestEventData::Test
        };

        let new_state = noop_builder(&mut state, &event);

        assert_eq!(new_state.0, "TEST");
    }
}
