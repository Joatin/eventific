use crate::event::Event;
use chrono::Utc;
use uuid::Uuid;

pub(crate) trait IntoEvent<D, M = ()> {
    fn into_event(
        self,
        aggregate_id: Uuid,
        base_event_id: u32,
        metadata: Option<M>,
    ) -> Vec<Event<D, M>>;
}

impl<D, M: Clone> IntoEvent<D, M> for Vec<D> {
    fn into_event(
        self,
        aggregate_id: Uuid,
        mut base_event_id: u32,
        metadata: Option<M>,
    ) -> Vec<Event<D, M>> {
        self.into_iter()
            .map(|data| {
                let e = Event {
                    aggregate_id,
                    event_id: base_event_id,
                    created_date: Utc::now(),
                    metadata: metadata.clone(),
                    payload: data,
                };
                base_event_id += 1;
                e
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::event::{Event, IntoEvent};
    use uuid::Uuid;

    #[derive(Debug, Clone, strum_macros::EnumIter)]
    enum TestEventData {
        Test,
    }

    fn setup_data() -> (Uuid, Vec<Event<TestEventData, ()>>) {
        let id = Uuid::parse_str("4355f3e6-be3e-4a91-a8a8-b967db878f5b").unwrap();

        let event_data = vec![
            TestEventData::Test,
            TestEventData::Test,
            TestEventData::Test,
        ];

        (id, event_data.into_event(id, 0, None))
    }

    #[test]
    fn into_event_should_set_correct_aggregate_id() {
        let (id, events) = setup_data();

        for event in events {
            assert_eq!(event.aggregate_id, id);
        }
    }
}
