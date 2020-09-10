use crate::event::{Event, EventData};
use crate::store::{Store, StoreError};
use slog::Logger;
use futures::{FutureExt, future};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use std::fmt::Debug;
use std::collections::HashMap;
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::stream;
use futures::stream::StreamExt;

/// Simple store that persists everything in runtime memory. This is great for prototyping and testing, but should not
/// be used in a real world scenario
#[derive(Default, Debug)]
pub struct MemoryStore<D: EventData, M> {
    events: Arc<Mutex<HashMap<String, Event<D, M>>>>
}

impl<D: EventData, M> MemoryStore<D, M> {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

impl<D: EventData, M> Clone for MemoryStore<D, M> {
    fn clone(&self) -> Self {
        Self {
            events: Arc::clone(&self.events)
        }
    }
}

impl<D: EventData, M: 'static + Send + Sync + Debug + Clone> Store<D, M> for MemoryStore<D, M> {
    fn init<'a>(&'a mut self, logger: &'a Logger, _service_name: &str) -> BoxFuture<'a, Result<(), StoreError<D, M>>> {
        info!(logger, "🧠  Setting up a new MemoryStore");
        warn!(logger, "🚨  The MemoryStore does not persist events longer than the lifetime of the process. It is recommended that you set up a more accurate store");
        future::ok(()).boxed()
    }

    fn save_events<'a>(&'a self, logger: &'a Logger, events: Vec<Event<D, M>>) -> BoxFuture<'a, Result<(), StoreError<D, M>>> {
        let mut map = self.events.lock().unwrap();
        for event in &events {
            if map.contains_key(&format!("{}:{}", event.aggregate_id, event.event_id)) {
                return future::err(StoreError::EventAlreadyExists(event.clone())).boxed()
            }
        }
        for event in events {
            let aggregate_id = event.aggregate_id;
            map.insert(format!("{}:{}", event.aggregate_id, event.event_id), event.clone());
            info!(logger, "Inserted event {:#?}", event; "aggregate_id" => aggregate_id.to_string());
        }
        future::ok(()).boxed()
    }

    fn events<'a>(&'a self, _logger: &'a Logger, _aggregate_id: Uuid) -> BoxFuture<'a, Result<BoxStream<'a, Result<Event<D, M>, StoreError<D, M>>>, StoreError<D, M>>> {
        let map = self.events.lock().unwrap();

        let result: BoxStream<_> = stream::iter(map.clone().into_iter())
            .map(|(_key, event)| Ok(event))
            .boxed();

        future::ok(result).boxed()
    }

    fn aggregate_ids<'a>(&'a self, _logger: &'a Logger) -> BoxFuture<'a, Result<BoxStream<'a, Result<Uuid, StoreError<D, M>>>, StoreError<D, M>>> {
        unimplemented!()
    }
}
