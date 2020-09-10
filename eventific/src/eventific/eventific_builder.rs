use crate::Eventific;
use crate::store::{Store};
use slog::Logger;
use crate::aggregate::{StateBuilder};
use crate::eventific::EventificError;
use crate::event::EventData;
use crate::component::Component;
use std::fmt::Debug;


/// A builder used to create a new Eventific instance
///
pub struct EventificBuilder<S: Send, D: EventData, St: Store<D, M>, M: 'static + Send + Sync + Debug + Clone = ()> {
    logger: Logger,
    components: Vec<Box<dyn Component<S, D, St, M>>>
}

impl<S: 'static + Default + Send, D: EventData + AsRef<str>, St: Store<D, M>, M: 'static + Send + Sync + Debug + Clone> EventificBuilder<S, D, St, M> {
    pub fn new() -> Self {
        let logger = Logger::root(
            slog::Discard,
            o!(),
        );

        Self {
            logger,
            components: vec![]
        }
    }

    pub fn logger(mut self, logger: &Logger) -> Self {
        self.logger = logger.clone();
        self
    }

    pub fn component(mut self, component: Box<dyn Component<S, D, St, M>>) -> Self {
        self.components.push(component);
        self
    }

    pub async fn build(self, service_name: &str, state_builder: StateBuilder<S, D, M>, mut store: St) -> Result<Eventific<S, D, St, M>, EventificError<D, M>> {
        let logger = self.logger.new(o!("service_name" => service_name.to_owned()));
        let components = self.components;

        info!(logger, "🚀  Starting Eventific");

        store.init(&logger, &service_name)
            .await
            .map_err(EventificError::StoreInitError)?;

        let eventific = Eventific::create(logger.clone(), store, state_builder, components).await?;

        Ok(eventific)
    }
}
