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

pub struct Eventific<S: Store<D>, D: 'static + Send + Sync + Debug, St> {
    store: S,
    state_builder: StateBuilder<St, D>,
    phantom_data: PhantomData<D>
}

impl<S: Store<D>, D: 'static + Send + Sync + Debug + Clone, St> Eventific<S, D, St> {
    pub fn new() -> EventificBuilder<MemoryStore<D>, D, St> {
        EventificBuilder::new()
    }
}

impl<S: Store<D>, D: 'static + Send + Sync + Debug + Clone, St: Default> Eventific<S, D, St> {

    pub(crate) fn create(store: S, state_builder: StateBuilder<St, D>) -> Self {
        Self {
            store,
            state_builder,
            phantom_data: PhantomData
        }
    }

    pub fn create_aggregate(&self, aggregate_id: Uuid, event_data: Vec<D>, metadata: Option<HashMap<String, String>>) -> impl Future<Item = (), Error=EventificError<D>> {
        let events = event_data.into_event(aggregate_id, 0, metadata);

        self.store.save_events(events).map_err(EventificError::StoreError)
    }

    pub fn aggregate(&self, aggregate_id: Uuid) -> impl Future<Item = Aggregate<St>, Error = EventificError<D>> {
        let state_builder = self.state_builder;
        self.store.events(aggregate_id)
            .map_err(EventificError::StoreError)
            .and_then(move |events| {
                Aggregate::from_events(state_builder, &events)
            })
    }

    pub fn add_events_to_aggregate<
        F: Fn(Aggregate<St>) -> IF,
        IF: IntoFuture<Item = Vec<D>, Error = Error, Future = FF>,
        FF: Future<Item = Vec<D>, Error = Error>
    >(&self, aggregate_id: Uuid, metadata: Option<HashMap<String, String>>, callback: F) -> impl Future<Item = (), Error = EventificError<D>> {
        futures::failed(EventificError::Unimplemented)
    }

    pub fn total_events(&self) -> impl Future<Item = u64, Error = EventificError<D>> {
        futures::failed(EventificError::Unimplemented)
    }

    pub fn total_events_for_aggregate(&self, aggregate_id: Uuid) -> impl Future<Item = u64, Error = EventificError<D>> {
        futures::failed(EventificError::Unimplemented)
    }

    pub fn total_aggregates(&self) -> impl Future<Item = u64, Error = EventificError<D>> {
        futures::failed(EventificError::Unimplemented)
    }

    pub fn all_aggregates(&self) -> impl Stream<Item = Aggregate<St>, Error = EventificError<D>> {
        futures::stream::once(Err(EventificError::Unimplemented))
    }

    pub fn updated_aggregates(&self) -> impl Stream<Item = Aggregate<St>, Error = EventificError<D>> {
        futures::stream::once(Err(EventificError::Unimplemented))
    }
}

#[cfg(test)]
mod test {
    use crate::Eventific;
    use crate::store::MemoryStore;
    use crate::aggregate::noop_builder;

    #[derive(Default)]
    struct FakeState;

    #[derive(Debug, Clone)]
    enum FakeEvent {
        Test
    }

    #[test]
    fn create_should_run_without_errors() {
        let _result: Eventific<MemoryStore<FakeEvent>, FakeEvent, FakeState> = Eventific::create(MemoryStore::new(), noop_builder);
    }
}
