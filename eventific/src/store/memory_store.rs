use crate::event::Event;
use crate::store::{Store, StoreError};
use slog::Logger;
use futures::{Future, Stream};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use std::fmt::Debug;
use std::collections::HashMap;

/// Simple store that persists everything in runtime memory. This is great for prototyping and testing, but should not
/// be used in a real world scenario
#[derive(Default, Debug)]
pub struct MemoryStore<D: 'static + Debug> {
    events: Arc<Mutex<HashMap<String, Event<D>>>>,
    logger: Option<Logger>
}

impl<D: Debug> MemoryStore<D> {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(HashMap::new())),
            logger: None
        }
    }
}

impl<D: 'static + Debug> Clone for MemoryStore<D> {
    fn clone(&self) -> Self {
        Self {
            events: Arc::clone(&self.events),
            logger: self.logger.clone()
        }
    }
}

impl<D: 'static + Send + Sync + Debug + Clone> Store<D> for MemoryStore<D> {
    fn init(&mut self, logger: &Logger, _service_name: &str) -> Box<Future<Item=(), Error=StoreError<D>> + Send> {
        self.logger.replace(logger.clone());
        info!(logger, "🧠  Setting up a new MemoryStore");
        warn!(logger, "🚨  The MemoryStore does not persist events longer than the lifetime of the process. It is recommended that you set up a more accurate store");
        Box::new(futures::done(Ok(())))
    }

    fn save_events(&self, events: Vec<Event<D>>) -> Box<Future<Item=(), Error=StoreError<D>> + Send> {
        let logger = self.logger.clone().expect("The store must be initialized");
        let mut map = self.events.lock().unwrap();
        for event in &events {
            if map.contains_key(&format!("{}:{}", event.aggregate_id, event.event_id)) {
                return Box::new(futures::failed(StoreError::EventAlreadyExists(event.clone())))
            }
        }
        for event in events {
            let aggregate_id = event.aggregate_id;
            map.insert(format!("{}:{}", event.aggregate_id, event.event_id), event.clone());
            info!(logger, "Inserted event {:#?}", event; "aggregate_id" => aggregate_id.to_string());
        }
        Box::new(futures::finished(()))
    }

    fn events(&self, aggregate_id: Uuid) -> Box<Future<Item=Vec<Event<D>>, Error=StoreError<D>> + Send> {
        let logger = self.logger.clone().expect("The store must be initialized");
        let map = self.events.lock().unwrap();

        let mut result = Vec::new();

        for (key, value) in map.iter() {
            if key.contains(&aggregate_id.to_string()) {
                result.push(value.clone());
            }
        }

        info!(logger, "Found events for aggregate {}, event: {:#?}", aggregate_id, &result);

        Box::new(futures::finished(result))
    }

    fn aggregate_ids(&self) -> Box<Stream<Item=Uuid, Error=StoreError<D>> + Send> {
        unimplemented!()
    }
}
