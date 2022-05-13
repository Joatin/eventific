use crate::storage::Storage;
use alloc::sync::Arc;
use uuid::Uuid;
use crate::aggregate::Aggregate;
use futures::stream::BoxStream;
use crate::{Event, Notifier, EventStoreBuilder};
use futures::{StreamExt, TryStreamExt};

/// Use the event store to either access aggregate roots, or to tail incoming events.
pub struct EventStore<P> {
    storage: Arc<dyn Storage<P>>,
    notifier: Option<Arc<dyn Notifier<P>>>
}

impl<P: Send + Sync> EventStore<P> {

    pub fn builder() -> EventStoreBuilder<P> {
        EventStoreBuilder::new()
    }

    pub(crate) fn new(storage: Arc<dyn Storage<P>>, notifier: Option<Arc<dyn Notifier<P>>>) -> Self {
        Self {
            storage,
            notifier
        }
    }

    /// Returns an aggregate root. No storage access is made at this point, and it's possible to get references to
    /// aggregates that hasn't been created yet
    pub fn aggregate(&self, id: Uuid) -> Aggregate<P> {
        Aggregate::new(id, Arc::clone(&self.storage))
    }

    /// Goes through all aggregates in the storage
    pub async fn all_aggregates(&self) -> BoxStream<'_, anyhow::Result<Aggregate<P>>> {
        let stream = self.storage.all_aggregate_ids().await;

        let stream = stream.map_ok(move |id| {
            Aggregate::new(id, Arc::clone(&self.storage))
        });

        stream.boxed()
    }

    /// The stream returns all events the are added to the store
    pub fn on_event(&self) -> anyhow::Result<BoxStream<Event<P>>> {
        todo!()
    }

    pub async fn total_events(&self) -> anyhow::Result<u64> {
        self.storage.total_events().await
    }
}

impl<P> Clone for EventStore<P> {
    fn clone(&self) -> Self {
        Self {
            storage: Arc::clone(&self.storage),
            notifier: self.notifier.clone()
        }
    }
}
