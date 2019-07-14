use futures::Future;
use futures::future::join_all;
use crate::Eventific;
use crate::store::{Store, MemoryStore};
use slog::Logger;
use crate::aggregate::{StateBuilder, noop_builder};
use crate::eventific::EventificError;
use std::fmt::Debug;
use crate::notification::{Sender, Listener, create_memory_notification_pair, MemorySender, MemoryListener};
use std::sync::Arc;

pub struct EventificBuilder<S, D: 'static + Send + Sync + Debug, St: Store<D>, Se: Sender, L: Listener> {
    store: St,
    state_builder: StateBuilder<S, D>,
    service_name: String,
    sender: Se,
    listener: L,
    logger: Logger
}

impl<S, D: 'static + Send + Sync + Debug + Clone> EventificBuilder<S, D, MemoryStore<D>, MemorySender, MemoryListener> {
    pub fn new() -> Self {
        let logger = Logger::root(
            slog::Discard,
            o!(),
        );

        let (sender, listener) = create_memory_notification_pair();

        Self {
            store: MemoryStore::new(),
            state_builder: noop_builder,
            service_name: "default".to_owned(),
            sender,
            listener,
            logger
        }
    }
}

impl<S: Default, D: 'static + Send + Sync + Debug + Clone, St: Store<D>, Se: 'static + Sender, L: 'static + Listener> EventificBuilder<S, D, St, Se, L> {

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
        let mut sender = self.sender;
        let mut listener = self.listener;
        let service_name = self.service_name;
        let logger = self.logger.new(o!("service_name" => service_name.to_owned()));

        print!("

    $$$$$$$$\\                             $$\\     $$\\  $$$$$$\\  $$\\
    $$  _____|                            $$ |    \\__|$$  __$$\\ \\__|
    $$ |  $$\\    $$\\  $$$$$$\\  $$$$$$$\\ $$$$$$\\   $$\\ $$ /  \\__|$$\\  $$$$$$$\\
    $$$$$\\\\$$\\  $$  |$$  __$$\\ $$  __$$\\\\_$$  _|  $$ |$$$$\\     $$ |$$  _____|
    $$  __|\\$$\\$$  / $$$$$$$$ |$$ |  $$ | $$ |    $$ |$$  _|    $$ |$$ /
    $$ |    \\$$$  /  $$   ____|$$ |  $$ | $$ |$$\\ $$ |$$ |      $$ |$$ |
    $$$$$$$$\\\\$  /   \\$$$$$$$\\ $$ |  $$ | \\$$$$  |$$ |$$ |      $$ |\\$$$$$$$\\
    \\________|\\_/     \\_______|\\__|  \\__|  \\____/ \\__|\\__|      \\__| \\_______|



");

        info!(logger, "🚀  Starting Eventific");


        store.init(&logger.clone(), &service_name)
            .map_err(EventificError::StoreInitError)
            .and_then(move |_| {
                sender.init(&logger.clone(), &service_name)
                    .map_err(EventificError::SendNotificationInitError)
                    .and_then(move |_| {
                        listener.init(&logger.clone(), &service_name)
                            .map_err(EventificError::SendNotificationInitError)
                            .and_then(move |_| {
                                info!(logger, "🤩  All setup and ready");

                                Ok(Eventific::create(store, state_builder, Arc::new(sender), Arc::new(listener)))
                            })
                    })
            })
    }
}
