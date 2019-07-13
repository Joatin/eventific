use crate::event::Event;

pub type StateBuilder<S, D> = fn(S, &Event<D>) -> S;

pub(crate) fn noop_builder<S, D>(state: S, _event: &Event<D>) -> S {
    state
}


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
        let state = TestState("TEST".to_owned());

        let event = Event {
            aggregate_id: Uuid::nil(),
            event_id: 0,
            created_date: Utc::now(),
            metadata: HashMap::new(),
            payload: TestEventData::Test
        };

        let new_state = noop_builder(state, &event);

        assert_eq!(new_state.0, "TEST");
    }
}
