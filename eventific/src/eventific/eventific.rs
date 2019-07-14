use crate::store::{Store, MemoryStore};
use crate::EventificBuilder;
use std::marker::PhantomData;
use crate::aggregate::{StateBuilder, Aggregate};
use std::fmt::Debug;
use crate::eventific::EventificError;
use futures::{Future, Stream, IntoFuture};
use uuid::Uuid;
use std::collections::HashMap;
use crate::event::IntoEvent;
use failure::Error;
use crate::notification::{Sender, Listener, MemorySender, MemoryListener};
use std::sync::Arc;

pub struct Eventific<S, D: 'static + Send + Sync + Debug, St: Store<D> = MemoryStore<D>> {
    store: St,
    state_builder: StateBuilder<S, D>,
    sender: Arc<Sender>,
    listener: Arc<Listener>,
    phantom_data: PhantomData<D>
}

impl<S: Default, D: 'static + Send + Sync + Debug + Clone, St: Store<D>> Clone for Eventific<S, D, St> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            state_builder: self.state_builder,
            sender: Arc::clone(&self.sender),
            listener: Arc::clone(&self.listener),
            phantom_data: PhantomData
        }
    }
}

impl<S, D: 'static + Send + Sync + Debug + Clone, St: Store<D>> Eventific<S, D, St> {
    pub fn new() -> EventificBuilder<S, D, MemoryStore<D>, MemorySender, MemoryListener> {
        EventificBuilder::new()
    }
}

impl<S: Default, D: 'static + Send + Sync + Debug + Clone, St: Store<D>> Eventific<S, D, St> {

    pub(crate) fn create(store: St, state_builder: StateBuilder<S, D>, sender: Arc<Sender>, listener: Arc<Listener>) -> Self {
        Self {
            store,
            state_builder,
            sender,
            listener,
            phantom_data: PhantomData
        }
    }

    pub fn create_aggregate(&self, aggregate_id: Uuid, event_data: Vec<D>, metadata: Option<HashMap<String, String>>) -> impl Future<Item = (), Error=EventificError<D>> {
        let events = event_data.into_event(aggregate_id, 0, metadata);
        let sender = Arc::clone(&self.sender);

        self.store.save_events(events)
            .map_err(EventificError::StoreError)
            .and_then(move |_| {
                sender.send(aggregate_id)
                    .map_err(EventificError::SendNotificationError)
            })
    }

    pub fn aggregate(&self, aggregate_id: Uuid) -> impl Future<Item = Aggregate<S>, Error = EventificError<D>> {
        let state_builder = self.state_builder;
        self.store.events(aggregate_id)
            .map_err(EventificError::StoreError)
            .and_then(move |events| {
                Aggregate::from_events(state_builder, &events)
            })
    }

    pub fn add_events_to_aggregate<
        F: Fn(Aggregate<S>) -> IF,
        IF: IntoFuture<Item = Vec<D>, Error = Error, Future = FF>,
        FF: Future<Item = Vec<D>, Error = Error>
    >(&self, aggregate_id: Uuid, metadata: Option<HashMap<String, String>>, callback: F) -> impl Future<Item = (), Error = EventificError<D>> {
        futures::failed(EventificError::Unimplemented)
    }

    pub fn total_events(&self) -> impl Future<Item = u64, Error = EventificError<D>> {
        self.store.total_events()
            .map_err(EventificError::StoreError)
    }

    pub fn total_events_for_aggregate(&self, aggregate_id: Uuid) -> impl Future<Item = u64, Error = EventificError<D>> {
        self.store.total_events_for_aggregate(aggregate_id)
            .map_err(EventificError::StoreError)
    }

    pub fn total_aggregates(&self) -> impl Future<Item = u64, Error = EventificError<D>> {
        self.store.total_aggregates()
            .map_err(EventificError::StoreError)
    }

    pub fn all_aggregates(&self) -> impl Stream<Item = Aggregate<S>, Error = EventificError<D>> {
        let eve = self.clone();
        self.store.aggregate_ids()
            .map_err(EventificError::StoreError)
            .and_then(move |uuid| {
                eve.aggregate(uuid)
            })
    }

    pub fn updated_aggregates(&self) -> impl Stream<Item = Aggregate<S>, Error = EventificError<D>> {
        let eve = self.clone();
        self.listener.listen()
            .map_err(EventificError::ListenNotificationError)
            .and_then(move |uuid| {
                eve.aggregate(uuid)
            })
    }
}

#[cfg(test)]
mod test {
    use crate::Eventific;
    use crate::store::MemoryStore;
    use crate::aggregate::noop_builder;
    use crate::notification::create_memory_notification_pair;
    use std::sync::Arc;

    #[derive(Default)]
    struct FakeState;

    #[derive(Debug, Clone)]
    enum FakeEvent {
        Test
    }

    #[test]
    fn create_should_run_without_errors() {
        let (sender, listener) = create_memory_notification_pair();
        let _result: Eventific<FakeState, FakeEvent, MemoryStore<FakeEvent>> = Eventific::create(MemoryStore::new(), noop_builder, Arc::new(sender), Arc::new(listener));
    }
}
