
#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_derive;

use eventific_dynamo::Region;
use eventific_dynamo::DynamoStore;
use tokio::runtime::Runtime;
use slog::Logger;
use eventific::store::Store;
use eventific::event::Event;
use uuid::Uuid;
use std::collections::HashMap;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TestEventData {
    Test
}

#[test]
#[cfg_attr(not(feature = "integration_tests"), ignore)]
fn it_should_store_and_retrieve_events() {
    let logger = Logger::root(
        slog::Discard,
        o!(),
    );
    let mut rt = Runtime::new().expect("Failed to create runtime");
    let region = Region::Custom {
        name: "eu-west-1".to_owned(),
        endpoint: "http://localhost:4564".to_owned(),
    };

    let mut store = DynamoStore::<TestEventData>::new_from_region(region, Some("eventific".to_owned()));

    rt.block_on(store.init(&logger, "eventific")).unwrap();

    let id = Uuid::new_v4();
    let events = vec![
        Event {
            aggregate_id: id,
            event_id: 0,
            created_date: Utc::now(),
            metadata: HashMap::new(),
            payload: TestEventData::Test
        }
    ];

    rt.block_on(store.save_events(events)).unwrap();

    let events = rt.block_on(store.events(id)).unwrap();

    assert_eq!(events.len(), 1);
}
