use crate::Eventific;
use crate::store::Store;
use std::fmt::Debug;
use strum::IntoEnumIterator;
use std::error::Error;
use uuid::Uuid;
use tokio::sync::broadcast::{ Sender as TokioSender };

#[async_trait::async_trait]
pub trait Receiver<
    St: Store<EventData = D, MetaData = M>,
    S: Send,
    D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator,
    M: 'static + Send + Sync + Debug,
>: 'static + Debug {
    async fn init(
        &mut self,
        eventific: &Eventific<St, S, D, M>,
        sender: TokioSender<Uuid>
    ) -> Result<(), Box<dyn Error + Send + Sync>>;

    /// A unique name for this component
    fn name(&self) -> &str;
}
