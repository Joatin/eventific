use crate::event::Event;
use crate::store::store_context::StoreContext;
use crate::store::{SaveEventsResult, Store};
use futures::future::BoxFuture;
use futures::stream;
use futures::stream::BoxStream;
use futures::stream::StreamExt;
use futures::{future};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Simple store that persists everything in runtime memory. This is great for prototyping and testing, but should not
/// be used in a real world scenario
#[derive(Default, Debug)]
pub struct MemoryStore<D, M> {
    events: Arc<Mutex<HashMap<String, Event<D, M>>>>,
}

impl<D, M> MemoryStore<D, M> {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<D, M> Clone for MemoryStore<D, M> {
    fn clone(&self) -> Self {
        Self {
            events: Arc::clone(&self.events),
        }
    }
}

#[async_trait::async_trait]
impl<D: 'static + Send + Sync + Debug + Clone, M: 'static + Send + Sync + Debug + Clone> Store
    for MemoryStore<D, M>
{
    type Error = MemoryStoreError<D, M>;
    type EventData = D;
    type MetaData = M;

    #[tracing::instrument]
    async fn init(&mut self, _context: StoreContext) -> Result<(), Self::Error> {
        info!("🧠  Setting up a new MemoryStore");
        warn!("🚨  The MemoryStore does not persist events longer than the lifetime of the process. It is recommended that you set up a more accurate store");
        Ok(())
    }

    #[tracing::instrument]
    async fn save_events(
        &self,
        _context: StoreContext,
        events: &Vec<Event<D, M>>,
    ) -> Result<SaveEventsResult, Self::Error> {
        let mut map = self.events.lock().unwrap();
        for event in events {
            if map.contains_key(&format!("{}:{}", event.aggregate_id, event.event_id)) {
                return Ok(SaveEventsResult::AlreadyExists)
            }
        }
        for event in events {
            let aggregate_id = event.aggregate_id;
            map.insert(format!("{}:{}", event.aggregate_id, event.event_id), event.clone());
            info!("Inserted event {:#?} for aggregate '{}'", event, aggregate_id.to_string());
        }
        Ok(SaveEventsResult::Success)
    }

    #[tracing::instrument]
    async fn events(
        &self,
        _context: StoreContext,
        _aggregate_id: Uuid,
    ) -> Result<BoxStream<'_, Result<Event<D, M>, Self::Error>>, Self::Error> {
        let map = self.events.lock().unwrap();

        let result: BoxStream<_> = stream::iter(map.clone().into_iter())
            .map(|(_key, event)| Ok(event))
            .boxed();

        Ok(result)
    }

    #[tracing::instrument]
    async fn aggregate_ids(
        &self,
        _context: StoreContext,
    ) -> Result<BoxStream<'_, Result<Uuid, Self::Error>>, Self::Error> {
        unimplemented!()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MemoryStoreError<D: Debug, M: Debug> {
    #[error("This event does already exist in this event store")]
    EventAlreadyExists(Event<D, M>),
}
