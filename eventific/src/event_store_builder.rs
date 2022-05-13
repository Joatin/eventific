use crate::event_store::EventStore;
use crate::memory_storage::MemoryStorage;
use crate::storage::Storage;
use crate::Notifier;
use alloc::sync::Arc;

/// Utility to create a fresh and new [EventStore]
pub struct EventStoreBuilder<P> {
    notifier: Option<Arc<dyn Notifier<P>>>,
}

impl<P: Send + Sync> EventStoreBuilder<P> {
    /// Creates a new [EventStoreBuilder]
    pub fn new() -> Self {
        Self { notifier: None }
    }

    pub fn with_notifier<N: 'static + Notifier<P>>(&mut self, notifer: N) -> &mut Self {
        self.notifier = Some(Arc::new(notifer));
        self
    }

    pub fn build<S: 'static + Storage<P>>(self, storage: S) -> EventStore<P> {
        EventStore::new(Arc::new(storage), self.notifier)
    }
}

impl<P: 'static + Send + Sync + Clone> EventStoreBuilder<P> {
    pub fn build_with_memory_storage(self) -> EventStore<P> {
        EventStore::new(Arc::new(MemoryStorage::new()), self.notifier)
    }
}

impl Default for EventStoreBuilder<()> {
    fn default() -> Self {
        Self::new()
    }
}
