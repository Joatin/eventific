use crate::Eventific;
use crate::store::{Store, MemoryStore};
use slog::Logger;
use crate::aggregate::{StateBuilder, noop_builder};
use crate::eventific::EventificError;
use crate::notification::{Sender, Listener, create_memory_notification_pair, MemorySender, MemoryListener};
use std::sync::Arc;
use colored::*;
use strum::IntoEnumIterator;
use crate::eventific::start_web_server::start_web_server;
use std::net::SocketAddr;
use std::str::FromStr;
use crate::event::EventData;

pub struct EventificBuilder<S: Send, D: EventData, St: Store<D>, Se: Sender, L: Listener> {
    store: St,
    state_builder: StateBuilder<S, D>,
    service_name: String,
    sender: Se,
    listener: L,
    logger: Logger,
    web_socket: String,
    web_port: u16,
    enable_web_server: bool
}

impl<S: Send, D: EventData> EventificBuilder<S, D, MemoryStore<D>, MemorySender, MemoryListener> {
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
            web_socket: "127.0.0.1".to_owned(),
            web_port: 9000,
            enable_web_server: true
        }
    }
}

impl<S: 'static + Default + Send, D: EventData + AsRef<str> + IntoEnumIterator<Iterator=DI>, DI: Iterator<Item=D>, St: Store<D>, Se: 'static + Sender, L: 'static + Listener> EventificBuilder<S, D, St, Se, L> {

    pub fn logger(mut self, logger: &Logger) -> Self {
        self.logger = logger.clone();
        self
    }

    pub fn service_name(mut self, service_name: &str) -> Self {
        self.service_name = service_name.to_owned();
        self
    }

    pub fn web_socket(mut self, web_socket: &str) -> Self {
        self.web_socket = web_socket.to_owned();
        self
    }

    pub fn web_port(mut self, web_port: u16) -> Self {
        self.web_port = web_port;
        self
    }

    pub fn enable_web_server(mut self, enable_web_server: bool) -> Self {
        self.enable_web_server = enable_web_server;
        self
    }

    pub fn state_builder(mut self, state_builder: StateBuilder<S, D>) -> Self {
        self.state_builder = state_builder;
        self
    }

    pub fn store<NSt: Store<D>>(self, store: NSt) -> EventificBuilder<S, D, NSt, Se, L> {
        EventificBuilder {
            store,
            state_builder: self.state_builder,
            service_name: self.service_name,
            sender: self.sender,
            listener: self.listener,
            logger: self.logger,
            web_socket: self.web_socket,
            web_port: self.web_port,
            enable_web_server: self.enable_web_server
        }
    }

    pub fn sender<NSe: Sender>(self, sender: NSe) -> EventificBuilder<S, D, St, NSe, L> {
        EventificBuilder {
            store: self.store,
            state_builder: self.state_builder,
            service_name: self.service_name,
            sender,
            listener: self.listener,
            logger: self.logger,
            web_socket: self.web_socket,
            web_port: self.web_port,
            enable_web_server: self.enable_web_server
        }
    }

    pub fn listener<NL: Listener>(self, listener: NL) -> EventificBuilder<S, D, St, Se, NL> {
        EventificBuilder {
            store: self.store,
            state_builder: self.state_builder,
            service_name: self.service_name,
            sender: self.sender,
            listener,
            logger: self.logger,
            web_socket: self.web_socket,
            web_port: self.web_port,
            enable_web_server: self.enable_web_server
        }
    }

    pub async fn build(self) -> Result<Eventific<S, D, St>, EventificError<D>> {
        let mut store = self.store;
        let state_builder = self.state_builder;
        let mut sender = self.sender;
        let mut listener = self.listener;
        let service_name = self.service_name;
        let logger = self.logger.new(o!("service_name" => service_name.to_owned()));
        let web_socket = self.web_socket;
        let web_port = self.web_port;
        let enable_web_server = self.enable_web_server;

        info!(logger, "🚀  Starting Eventific");

        store.init(&logger, &service_name)
            .await
            .map_err(EventificError::StoreInitError)?;

        sender.init(&logger, &service_name)
            .await
            .map_err(EventificError::SendNotificationInitError)?;

        listener.init(&logger, &service_name)
            .await
            .map_err(EventificError::SendNotificationInitError)?;

        let eventific = Eventific::create(logger.clone(), store, state_builder, Arc::new(sender), Arc::new(listener));


        if enable_web_server {
            tokio::spawn(start_web_server(logger.clone(), SocketAddr::from_str(&format!("{}:{}", web_socket, web_port)).expect("Provided socket or port is not valid!")));
        } else {
            info!(logger, "Web server disabled");
        }

        info!(logger, "Available events are:");
        info!(logger, "");
        for event in D::iter() {
            info!(logger, "{}", event.as_ref());
        }
        info!(logger, "");

        info!(logger, "🤩  All setup and ready");


        Ok(eventific)
    }
}
