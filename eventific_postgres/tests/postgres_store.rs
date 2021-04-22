#[macro_use]
extern crate serde_derive;

use chrono::Utc;
use eventific::store::{Store, StoreContext};
use eventific::{Event, EventData};
use eventific_postgres::PostgresStore;
use futures::lazy;
use futures::stream::Stream;
use slog::Logger;
use std::collections::HashMap;
use tokio::runtime::Runtime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TestEventData {
    Test,
}

impl EventData for TestEventData {}

const CONNECTION_STRING: &'static str = "postgresql://admin:password@localhost:5432/postgres";

#[test]
#[cfg_attr(not(feature = "integration_tests"), ignore)]
fn it_should_store_and_retrieve_events() {
    let mut rt = Runtime::new().expect("Failed to create runtime");

    let mut store = PostgresStore::<TestEventData, ()>::new(CONNECTION_STRING);

    rt.block_on(store.init(StoreContext { service_name: "".to_string() }, "eventific")).unwrap();

    let id = Uuid::new_v4();
    let events = vec![
        Event {
            aggregate_id: id,
            event_id: 0,
            created_date: Utc::now(),
            metadata: Some(()),
            payload: TestEventData::Test,
        },
        Event {
            aggregate_id: id,
            event_id: 1,
            created_date: Utc::now(),
            metadata: Some(()),
            payload: TestEventData::Test,
        },
        Event {
            aggregate_id: id,
            event_id: 2,
            created_date: Utc::now(),
            metadata: Some(()),
            payload: TestEventData::Test,
        },
    ];

    rt.block_on(store.save_events(StoreContext { service_name: "".to_string() }, events)).unwrap();

    let events = rt.block_on(store.events(id)).unwrap();

    assert_eq!(events.len(), 3);
    assert_eq!(events[0].aggregate_id, id);
    assert_eq!(events[1].aggregate_id, id);
    assert_eq!(events[2].aggregate_id, id);

    assert_eq!(events[0].event_id, 0);
    assert_eq!(events[1].event_id, 1);
    assert_eq!(events[2].event_id, 2);
}

#[test]
#[cfg_attr(not(feature = "integration_tests"), ignore)]
fn it_should_retrieve_all_aggregate_ids() {
    let logger = Logger::root(slog::Discard, o!());
    let mut rt = Runtime::new().expect("Failed to create runtime");

    let mut store = PostgresStore::<TestEventData>::new(CONNECTION_STRING);

    rt.block_on(store.init(&logger, "eventific")).unwrap();

    let events1 = vec![Event {
        aggregate_id: Uuid::new_v4(),
        event_id: 0,
        created_date: Utc::now(),
        metadata: HashMap::new(),
        payload: TestEventData::Test,
    }];
    let events2 = vec![Event {
        aggregate_id: Uuid::new_v4(),
        event_id: 0,
        created_date: Utc::now(),
        metadata: HashMap::new(),
        payload: TestEventData::Test,
    }];
    let events3 = vec![Event {
        aggregate_id: Uuid::new_v4(),
        event_id: 0,
        created_date: Utc::now(),
        metadata: HashMap::new(),
        payload: TestEventData::Test,
    }];

    rt.block_on(store.save_events(events1)).unwrap();
    rt.block_on(store.save_events(events2)).unwrap();
    rt.block_on(store.save_events(events3)).unwrap();

    let ids = rt
        .block_on(lazy(move || store.aggregate_ids().collect()))
        .unwrap();

    assert!(ids.len() >= 3);
}

#[test]
#[cfg_attr(not(feature = "integration_tests"), ignore)]
fn it_should_return_total_aggregates() {
    let logger = Logger::root(slog::Discard, o!());
    let mut rt = Runtime::new().expect("Failed to create runtime");

    let mut store = PostgresStore::<TestEventData>::new(CONNECTION_STRING);

    rt.block_on(store.init(&logger, "eventific")).unwrap();

    let events1 = vec![Event {
        aggregate_id: Uuid::new_v4(),
        event_id: 0,
        created_date: Utc::now(),
        metadata: HashMap::new(),
        payload: TestEventData::Test,
    }];
    let events2 = vec![Event {
        aggregate_id: Uuid::new_v4(),
        event_id: 0,
        created_date: Utc::now(),
        metadata: HashMap::new(),
        payload: TestEventData::Test,
    }];
    let events3 = vec![Event {
        aggregate_id: Uuid::new_v4(),
        event_id: 0,
        created_date: Utc::now(),
        metadata: HashMap::new(),
        payload: TestEventData::Test,
    }];

    rt.block_on(store.save_events(events1)).unwrap();
    rt.block_on(store.save_events(events2)).unwrap();
    rt.block_on(store.save_events(events3)).unwrap();

    let count = rt.block_on(store.total_aggregates()).unwrap();

    assert!(count >= 3);
}

#[test]
#[cfg_attr(not(feature = "integration_tests"), ignore)]
fn it_should_return_total_events() {
    let logger = Logger::root(slog::Discard, o!());
    let mut rt = Runtime::new().expect("Failed to create runtime");

    let mut store = PostgresStore::<TestEventData>::new(CONNECTION_STRING);

    rt.block_on(store.init(&logger, "eventific")).unwrap();

    let events1 = vec![Event {
        aggregate_id: Uuid::new_v4(),
        event_id: 0,
        created_date: Utc::now(),
        metadata: HashMap::new(),
        payload: TestEventData::Test,
    }];
    let events2 = vec![Event {
        aggregate_id: Uuid::new_v4(),
        event_id: 0,
        created_date: Utc::now(),
        metadata: HashMap::new(),
        payload: TestEventData::Test,
    }];
    let events3 = vec![Event {
        aggregate_id: Uuid::new_v4(),
        event_id: 0,
        created_date: Utc::now(),
        metadata: HashMap::new(),
        payload: TestEventData::Test,
    }];

    rt.block_on(store.save_events(events1)).unwrap();
    rt.block_on(store.save_events(events2)).unwrap();
    rt.block_on(store.save_events(events3)).unwrap();

    let count = rt.block_on(store.total_events()).unwrap();

    assert!(count >= 3);
}

#[test]
#[cfg_attr(not(feature = "integration_tests"), ignore)]
fn it_should_return_total_events_for_aggregate() {
    let logger = Logger::root(slog::Discard, o!());
    let mut rt = Runtime::new().expect("Failed to create runtime");

    let mut store = PostgresStore::<TestEventData>::new(CONNECTION_STRING);

    rt.block_on(store.init(&logger, "eventific")).unwrap();

    let id = Uuid::new_v4();
    let events = vec![
        Event {
            aggregate_id: id,
            event_id: 0,
            created_date: Utc::now(),
            metadata: HashMap::new(),
            payload: TestEventData::Test,
        },
        Event {
            aggregate_id: id,
            event_id: 1,
            created_date: Utc::now(),
            metadata: HashMap::new(),
            payload: TestEventData::Test,
        },
        Event {
            aggregate_id: id,
            event_id: 2,
            created_date: Utc::now(),
            metadata: HashMap::new(),
            payload: TestEventData::Test,
        },
    ];

    rt.block_on(store.save_events(events)).unwrap();

    let count = rt.block_on(store.total_events_for_aggregate(id)).unwrap();

    assert_eq!(count, 3);
}
