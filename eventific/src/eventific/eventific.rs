use crate::store::{Store, MemoryStore, StoreError};
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
use std::sync::{Arc, Mutex};
use slog::Logger;
use tokio::runtime::Runtime;
use tokio::runtime::Builder;
use futures::future::{loop_fn, Loop};
use std::time::{Duration, Instant};
use std::ops::Add;
use tokio::timer::Delay;

pub struct Eventific<S, D: 'static + Send + Sync + Debug, St: Store<D> = MemoryStore<D>> {
    logger: Logger,
    runtime: Arc<Mutex<Runtime>>,
    store: St,
    state_builder: StateBuilder<S, D>,
    sender: Arc<Sender>,
    listener: Arc<Listener>,
    phantom_data: PhantomData<D>
}

impl<S, D: 'static + Send + Sync + Debug, St: Store<D>> Clone for Eventific<S, D, St> {
    fn clone(&self) -> Self {
        Self {
            logger: self.logger.clone(),
            runtime: Arc::clone(&self.runtime),
            store: self.store.clone(),
            state_builder: self.state_builder,
            sender: Arc::clone(&self.sender),
            listener: Arc::clone(&self.listener),
            phantom_data: PhantomData
        }
    }
}

impl<S, D: 'static + Send + Sync + Debug, St: Store<D>> Eventific<S, D, St> {

    pub fn get_logger(&self) -> &Logger {
        &self.logger
    }

    pub fn spawn<F: Future<Item = (), Error = ()> + Send + 'static>(&self, future: F) {
        let mut lock = self.runtime.lock().unwrap();
        lock.spawn(future);
    }
}

impl<S, D: 'static + Send + Sync + Debug + Clone, St: Store<D>> Eventific<S, D, St> {
    pub fn new() -> EventificBuilder<S, D, MemoryStore<D>, MemorySender, MemoryListener> {
        EventificBuilder::new()
    }
}

impl<S: Default, D: 'static + Send + Sync + Debug + Clone, St: Store<D>> Eventific<S, D, St> {
    const MAX_ATTEMPTS: u64 = 10;

    pub(crate) fn create(logger: Logger, store: St, state_builder: StateBuilder<S, D>, sender: Arc<Sender>, listener: Arc<Listener>) -> Self {
        let runtime = Builder::new()
            .name_prefix("eventific")
            .build()
            .expect("Failed to create thread pool");
        Self {
            logger,
            runtime: Arc::new(Mutex::new(runtime)),
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
    >(&self, aggregate_id: Uuid, _metadata: Option<HashMap<String, String>>, callback: F) -> impl Future<Item = (), Error = EventificError<D>> {
        loop_fn((0, self.clone(), aggregate_id, callback), |(attempts, eventific, id, call)| {
            Delay::new(Instant::now().add(Duration::from_millis(100 * attempts)))
                .map_err(|e| EventificError::Unknown(format_err!("{}", e)))
                .and_then(move |_| {
                    let state_builder = eventific.state_builder;
                    eventific.store.events(id)
                        .map_err(EventificError::StoreError)
                        .and_then(move |events| {
                            Aggregate::from_events(state_builder, &events)
                        })
                        .and_then(move |aggregate| {
                            let next_version = (aggregate.version + 1) as u32;
                            call(aggregate)
                                .into_future()
                                .map_err(EventificError::ValidationError)
                                .and_then(move |event_data| {
                                    let events = event_data.into_event(id, next_version, None);
                                    eventific.store.save_events(events)
                                        .then(move |res| {
                                            match res {
                                                Ok(_) => {
                                                    Ok(Loop::Break(()))
                                                },
                                                Err(err) => {
                                                    if let StoreError::EventAlreadyExists(_) = err {
                                                        if attempts < Self::MAX_ATTEMPTS {
                                                            Ok(Loop::Continue((attempts, eventific, id, call)))
                                                        } else {
                                                            Err(EventificError::StoreError(err))
                                                        }
                                                    } else {
                                                        Err(EventificError::StoreError(err))
                                                    }
                                                },
                                            }
                                        })
                                })
                        })
                })
        })
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


impl<S: 'static + Default + Send, D: 'static + Send + Sync + Debug + Clone, St: Store<D> + Sync> Eventific<S, D, St> {
    // GRPC //

    #[cfg(feature = "rpc")]
    pub fn grpc_create_aggregate<
        Input: 'static + Send,
        Resp: 'static + Send,
        IC: 'static + FnOnce(&Input) -> &str,
        VC: 'static + FnOnce(&Input) -> Result<Vec<D>, Error> + Send,
        RC: 'static + FnOnce() -> Resp + Send
    >(
        &self,
        ctx: grpc::RequestOptions,
        input: Input,
        id_callback: IC,
        event_callback: VC,
        result_callback: RC
    ) -> grpc::SingleResponse<Resp> {
        crate::grpc::grpc_command_new_aggregate(
            self,
            ctx,
            input,
            id_callback,
            event_callback,
            result_callback
        )
    }

    #[cfg(feature = "rpc")]
    pub fn grpc_add_events_to_aggregate<
        Input: 'static + Sync + Send + Clone,
        Resp: 'static + Send,
        IC: 'static + FnOnce(&Input) -> &str,
        VC: 'static + Fn(&Input, Aggregate<S>) -> IF + Send,
        RC: 'static + FnOnce() -> Resp + Send,
        IF: 'static + IntoFuture<Item=Vec<D>, Error=Error, Future=FF>,
        FF: 'static + Future<Item=Vec<D>, Error=Error> + Send
    >(
        &self,
        ctx: grpc::RequestOptions,
        input: Input,
        id_callback: IC,
        event_callback: VC,
        result_callback: RC
    ) -> grpc::SingleResponse<Resp> {
        crate::grpc::grpc_command_existing_aggregate(
            self,
            ctx,
            input,
            id_callback,
            event_callback,
            result_callback
        )
    }
}

#[cfg(test)]
mod test {
    use crate::Eventific;
    use crate::store::MemoryStore;
    use crate::aggregate::noop_builder;
    use crate::notification::create_memory_notification_pair;
    use std::sync::Arc;
    use slog::Logger;

    #[derive(Default)]
    struct FakeState;

    #[derive(Debug, Clone)]
    enum FakeEvent {
        Test
    }

    #[test]
    fn create_should_run_without_errors() {
        let logger = Logger::root(
            slog::Discard,
            o!(),
        );
        let (sender, listener) = create_memory_notification_pair();
        let _result: Eventific<FakeState, FakeEvent, MemoryStore<FakeEvent>> = Eventific::create(logger, MemoryStore::new(), noop_builder, Arc::new(sender), Arc::new(listener));
    }
}
