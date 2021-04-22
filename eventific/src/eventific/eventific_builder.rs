use crate::aggregate::StateBuilder;
use crate::component::Component;
use crate::eventific::EventificError;
use crate::store::Store;
use crate::Eventific;
use std::fmt::Debug;
use strum::IntoEnumIterator;

/// A builder used to create a new Eventific instance
#[derive(Debug)]
pub struct EventificBuilder<
    St: Store<EventData = D, MetaData = M>,
    S: Send,
    D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator,
    M: 'static + Send + Sync + Debug + Clone = (),
> {
    components: Vec<Box<dyn Component<St, S, D, M>>>,
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
        }
    }

    #[tracing::instrument(skip(component, self))]
    pub fn component(mut self, component: Box<dyn Component<St, S, D, M>>) -> Self {
        self.components.push(component);
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
        )
        .await?;

        Ok(eventific)
    }
}
