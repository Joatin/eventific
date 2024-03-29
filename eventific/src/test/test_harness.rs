use crate::aggregate::StateBuilder;
use crate::event::IntoEvent;
use std::fmt::Debug;
use uuid::Uuid;

pub struct TestHarness<S: Debug, D, M> {
    state_builder: StateBuilder<S, D, M>,
    current_state: S,
    next_event_id: u32,
}

impl<S: Default + Debug + PartialEq + Clone, D: Debug, M: Debug + Clone> TestHarness<S, D, M> {
    pub fn new(state_builder: StateBuilder<S, D, M>) -> Self {
        Self {
            state_builder,
            current_state: S::default(),
            next_event_id: 0,
        }
    }

    pub fn given_state(&mut self, state: S) -> &mut Self {
        self.current_state = state;
        self
    }

    pub fn expect_state(&mut self, state: S) -> &mut Self {
        assert_eq!(self.current_state, state, "The current state and the expected state are not equal, the current state was: \n\n{:#?}\n\n", self.current_state);
        self
    }

    pub fn apply_events(&mut self, event_data: Vec<D>, metadata: Option<M>) -> &mut Self {
        let events = event_data.into_event(Uuid::default(), self.next_event_id, metadata);
        self.next_event_id += events.len() as u32;
        for event in events {
            (self.state_builder)((&mut self.current_state, &event))
        }
        self
    }
}

#[cfg(test)]
mod test {
    use crate::event::Event;
    use crate::test::TestHarness;

    #[derive(Debug, Clone, strum_macros::EnumIter)]
    enum TestEventData {
        Created,
    }

    #[derive(Debug, PartialEq, Default, Clone)]
    struct State {
        created: bool,
    }

    fn state_builder((state, event): (&mut State, &Event<TestEventData, ()>)) {
        match event.payload {
            TestEventData::Created => {
                state.created = true;
            }
        }
    }

    #[test]
    fn it_should_update_state() {
        let _harness = TestHarness::new(state_builder)
            .given_state(State { created: false })
            .apply_events(vec![TestEventData::Created])
            .expect_state(State { created: true });
    }
}
