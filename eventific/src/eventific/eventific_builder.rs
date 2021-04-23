use crate::aggregate::StateBuilder;
use crate::component::Component;
use crate::eventific::EventificError;
use crate::store::Store;
use crate::Eventific;
use std::fmt::Debug;
use strum::IntoEnumIterator;
use crate::notification::{Receiver, Sender};

/// A builder used to create a new Eventific instance
#[derive(Debug)]
pub struct EventificBuilder<
    St: Store<EventData = D, MetaData = M>,
    S: Send,
    D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator,
    M: 'static + Send + Sync + Debug + Clone = (),
> {
    components: Vec<Box<dyn Component<St, S, D, M>>>,
    receivers: Vec<Box<dyn Receiver<St, S, D, M>>>,
    senders: Vec<Box<dyn Sender<St, S, D, M>>>,
}

impl<
        St: Store<EventData = D, MetaData = M>,
        S: 'static + Default + Send + Debug,
        D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator + AsRef<str>,
        M: 'static + Send + Sync + Debug + Clone,
    > EventificBuilder<St, S, D, M>
{

    #[tracing::instrument]
    pub fn new() -> Self {

        Self {
            components: vec![],
            receivers: vec![],
            senders: vec![],
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn component<C: Component<St, S, D, M>>(mut self, component: C) -> Self {
        self.components.push(Box::new(component));
        self
    }

    #[tracing::instrument(skip(self))]
    pub fn receiver<R: Receiver<St, S, D, M>>(mut self, receiver: R) -> Self {
        self.receivers.push(Box::new(receiver));
        self
    }

    #[tracing::instrument(skip(self))]
    pub fn sender<Se: Sender<St, S, D, M>>(mut self, sender: Se) -> Self {
        self.senders.push(Box::new(sender));
        self
    }

    #[tracing::instrument(skip(state_builder, self))]
    pub async fn build(
        self,
        service_name: &str,
        state_builder: StateBuilder<S, D, M>,
        store: St,
    ) -> Result<Eventific<St, S, D, M>, EventificError<St::Error, D, M>> {
        let eventific = Eventific::new(
            store,
            service_name,
            state_builder,
            self.components,
            self.receivers,
            self.senders
        )
        .await?;

        Ok(eventific)
    }
}
