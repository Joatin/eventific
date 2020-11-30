use crate::aggregate::StateBuilder;
use crate::component::Component;
use crate::eventific::EventificError;
use crate::store::Store;
use crate::Eventific;
use slog::Logger;
use std::fmt::Debug;
use strum::IntoEnumIterator;

/// A builder used to create a new Eventific instance
///
pub struct EventificBuilder<
    St: Store<EventData = D, MetaData = M>,
    S: Send,
    D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator,
    M: 'static + Send + Sync + Debug + Clone = (),
> {
    logger: Logger,
    components: Vec<Box<dyn Component<St, S, D, M>>>,
}

impl<
        St: Store<EventData = D, MetaData = M>,
        S: 'static + Default + Send,
        D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator + AsRef<str>,
        M: 'static + Send + Sync + Debug + Clone,
    > EventificBuilder<St, S, D, M>
{
    pub fn new() -> Self {
        let logger = Logger::root(slog::Discard, o!());

        Self {
            logger,
            components: vec![],
        }
    }

    pub fn logger(mut self, logger: &Logger) -> Self {
        self.logger = logger.clone();
        self
    }

    pub fn component(mut self, component: Box<dyn Component<St, S, D, M>>) -> Self {
        self.components.push(component);
        self
    }

    pub async fn build(
        self,
        service_name: &str,
        state_builder: StateBuilder<S, D, M>,
        store: St,
    ) -> Result<Eventific<St, S, D, M>, EventificError<St::Error, D, M>> {
        let eventific = Eventific::new(
            self.logger,
            store,
            service_name,
            state_builder,
            self.components,
        )
        .await?;

        Ok(eventific)
    }
}
