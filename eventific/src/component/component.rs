use crate::store::Store;
use crate::Eventific;
use futures::future::BoxFuture;
use slog::Logger;
use std::any::Any;
use std::error::Error;
use std::fmt::Debug;
use strum::IntoEnumIterator;

/// A trait for code that brings additional functionality to eventific
pub trait Component<
    St: Store<EventData = D, MetaData = M>,
    S: Send,
    D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator,
    M: 'static + Send + Sync + Debug,
>: Any
{
    fn init(
        &mut self,
        logger: Logger,
        eventific: Eventific<St, S, D, M>,
    ) -> BoxFuture<Result<(), Box<dyn Error + Send + Sync>>>;

    /// A unique name for this component
    fn component_name(&self) -> &str;
}
