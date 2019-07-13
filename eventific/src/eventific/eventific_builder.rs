use futures::Future;
use crate::Eventific;
use crate::store::{Store, MemoryStore};
use slog::Logger;
use crate::aggregate::{StateBuilder, noop_builder};
use crate::eventific::EventificError;
use std::fmt::Debug;

pub struct EventificBuilder<S: Store<D>, D: 'static + Send + Sync + Debug, St> {
    store: S,
    state_builder: StateBuilder<St, D>,
    service_name: String,
    logger: Logger
}

impl<D: 'static + Send + Sync + Debug + Clone, St> EventificBuilder<MemoryStore<D>, D, St> {
    pub fn new() -> Self {
        let logger = Logger::root(
            slog::Discard,
            o!(),
        );

        Self {
            store: MemoryStore::new(),
            state_builder: noop_builder,
            service_name: "default".to_owned(),
            logger
        }
    }
}

impl<S: Store<D>, D: 'static + Send + Sync + Debug + Clone, St: Default> EventificBuilder<S, D, St> {

    pub fn logger(mut self, logger: &Logger) -> Self {
        self.logger = logger.clone();
        self
    }

    pub fn service_name(mut self, service_name: &str) -> Self {
        self.service_name = service_name.to_owned();
        self
    }

    pub fn start(self) -> impl Future<Item = Eventific<S, D, St>, Error = EventificError<D>> {
        let mut store = self.store;
        let state_builder = self.state_builder;
        let logger = self.logger.new(o!("service_name" => self.service_name.to_owned()));

        info!(logger, "Starting Eventific");

        store.init(&logger.clone(), &self.service_name)
            .map_err(EventificError::StoreInitError)
            .and_then(move |()| {
            Ok(Eventific::create(store, state_builder))
        })
    }
}
