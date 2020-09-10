use slog::Logger;
use futures::future::BoxFuture;
use crate::Eventific;
use crate::event::EventData;
use crate::store::Store;
use std::fmt::Debug;

/// A trait for code that brings additional functionality to eventific
pub trait Component<S: Send, D: EventData, St: Store<D, M>, M: 'static + Send + Sync + Debug> {

    fn init(&mut self, logger: Logger, eventific: Eventific<S, D, St, M>) -> BoxFuture<Result<(), failure::Error>>;

    /// A unique name for this component
    fn component_name(&self) -> &'static str;
}
