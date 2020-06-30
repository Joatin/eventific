use crate::event::{Event, EventData};
use uuid::Uuid;
use std::collections::HashMap;
use chrono::Utc;

pub(crate) trait IntoEvent<D: EventData> {
    fn into_event(self, aggregate_id: Uuid, base_event_id: u32, metadata: Option<HashMap<String, String>>) -> Vec<Event<D>>;
}

impl<D: EventData> IntoEvent<D> for Vec<D> {
    fn into_event(self, aggregate_id: Uuid, mut base_event_id: u32, metadata: Option<HashMap<String, String>>) -> Vec<Event<D>> {
        self.into_iter()
            .map(|data| {
                let e = Event {
                    aggregate_id,
                    event_id: base_event_id,
                    created_date: Utc::now(),
                    metadata: metadata.clone().unwrap_or_default(),
                    payload: data
                };
                base_event_id += 1;
                e
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::event::{IntoEvent, Event};
    use uuid::Uuid;

    enum TestEventData {
        Test
    }

    fn setup_data() -> (Uuid, Vec<Event<TestEventData>>) {
        let id = Uuid::parse_str("4355f3e6-be3e-4a91-a8a8-b967db878f5b").unwrap();

        let event_data = vec![
            TestEventData::Test,
            TestEventData::Test,
            TestEventData::Test
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
