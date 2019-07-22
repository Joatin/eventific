use futures::Future;
use crate::Eventific;
use crate::store::{Store, MemoryStore};
use slog::Logger;
use crate::aggregate::{StateBuilder, noop_builder};
use crate::eventific::EventificError;
use std::fmt::Debug;
use crate::notification::{Sender, Listener, create_memory_notification_pair, MemorySender, MemoryListener};
use std::sync::Arc;
use colored::*;

pub struct EventificBuilder<S, D: 'static + Send + Sync + Debug, St: Store<D>, Se: Sender, L: Listener> {
    store: St,
    state_builder: StateBuilder<S, D>,
    service_name: String,
    sender: Se,
    listener: L,
    logger: Logger,
    #[cfg(feature = "playground")]
    playground: bool,
    #[cfg(feature = "with_grpc")]
    grpc_services: Vec<Box<dyn Fn(Eventific<S, D, St>) -> grpc::rt::ServerServiceDefinition + Send>>,
    #[cfg(feature = "with_grpc")]
    grpc_port: u16
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
            logger,
            #[cfg(feature = "playground")]
            playground: false,
            #[cfg(feature = "with_grpc")]
            grpc_services: Vec::new(),
            #[cfg(feature = "with_grpc")]
            grpc_port: 50051
        }
    }
}

impl<S: 'static + Default, D: 'static + Send + Sync + Debug + Clone, St: Store<D>, Se: 'static + Sender, L: 'static + Listener> EventificBuilder<S, D, St, Se, L> {

    pub fn logger(mut self, logger: &Logger) -> Self {
        self.logger = logger.clone();
        self
    }

    pub fn service_name(mut self, service_name: &str) -> Self {
        self.service_name = service_name.to_owned();
        self
    }

    pub fn store<NSt: Store<D>>(self, store: NSt) -> EventificBuilder<S, D, NSt, Se, L> {

        #[cfg(feature = "with_grpc")]
        {
            if !self.grpc_services.is_empty() {
                panic!("You can only add command handlers AFTER you have changed the store")
            }
        }

        EventificBuilder {
            store,
            state_builder: self.state_builder,
            service_name: self.service_name,
            sender: self.sender,
            listener: self.listener,
            logger: self.logger,
            #[cfg(feature = "playground")]
            playground: self.playground,
            #[cfg(feature = "with_grpc")]
            grpc_services: Vec::new(),
            #[cfg(feature = "with_grpc")]
            grpc_port: 50051
        }
    }

    #[cfg(feature = "with_grpc")]
    pub fn with_grpc_service<
        HC: 'static + Send + Fn(Eventific<S, D, St>) -> grpc::rt::ServerServiceDefinition
    >(
        mut self,
        service_callback: HC
    ) -> Self {
        self.grpc_services.push(Box::new(service_callback));
        self
    }

    pub fn grpc_port(mut self, port: u16) -> Self {
        self.grpc_port = port;
        self
    }

    #[cfg(feature = "playground")]
    pub fn enable_playground(mut self) -> Self {
        self.playground = true;
        self
    }

    pub fn start(self) -> impl Future<Item = Eventific<S, D, St>, Error = EventificError<D>> {
        let mut store = self.store;
        let state_builder = self.state_builder;
        let mut sender = self.sender;
        let mut listener = self.listener;
        let service_name = self.service_name;
        #[cfg(feature = "playground")]
        let use_playground = self.playground;
        #[cfg(feature = "with_grpc")]
        let grpc_command_handlers = self.grpc_services;
        #[cfg(feature = "with_grpc")]
        let grpc_port = self.grpc_port;
        let logger = self.logger.new(o!("service_name" => service_name.to_owned()));

        print!("{}", "

    $$$$$$$$\\                             $$\\     $$\\  $$$$$$\\  $$\\
    $$  _____|                            $$ |    \\__|$$  __$$\\ \\__|
    $$ |  $$\\    $$\\  $$$$$$\\  $$$$$$$\\ $$$$$$\\   $$\\ $$ /  \\__|$$\\  $$$$$$$\\
    $$$$$\\\\$$\\  $$  |$$  __$$\\ $$  __$$\\\\_$$  _|  $$ |$$$$\\     $$ |$$  _____|
    $$  __|\\$$\\$$  / $$$$$$$$ |$$ |  $$ | $$ |    $$ |$$  _|    $$ |$$ /
    $$ |    \\$$$  /  $$   ____|$$ |  $$ | $$ |$$\\ $$ |$$ |      $$ |$$ |
    $$$$$$$$\\\\$  /   \\$$$$$$$\\ $$ |  $$ | \\$$$$  |$$ |$$ |      $$ |\\$$$$$$$\\
    \\________|\\_/     \\_______|\\__|  \\__|  \\____/ \\__|\\__|      \\__| \\_______|



".green());

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
                                let eventific = Eventific::create(logger.clone(), store, state_builder, Arc::new(sender), Arc::new(listener));

                                #[cfg(feature = "playground")]
                                {
                                    if use_playground {
                                        tokio::spawn(crate::playground::start_playground_server(&logger, &eventific));
                                    }
                                }

                                #[cfg(feature = "with_grpc")]
                                {
                                    if !grpc_command_handlers.is_empty() {
                                        crate::grpc::start_grpc_server(&logger, eventific.clone(), grpc_port, grpc_command_handlers)?;
                                    }
                                }

                                info!(logger, "🤩  All setup and ready");


                                Ok(eventific)
                            })
                    })
            })
    }
}
