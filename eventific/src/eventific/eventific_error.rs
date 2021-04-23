use crate::event::Event;
use crate::store::Store;
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum EventificError<StoreError: 'static + std::error::Error, D: Debug, M: Debug> {
    #[error("Apply event callback returned validation failure, internal error was {0}")]
    ValidationError(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("The events seems to be missing a event, or they appear in the wrong order, the event was {0:?}")]
    InconsistentEventChain(Event<D, M>),
    #[error("Failed while setting up store, internal error was: {0}")]
    StoreInitError(#[source] StoreError),
    #[error("Failed while setting up eventific, internal error was: {0}")]
    InitError(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Store failed, internal error was: {0}")]
    StoreError(#[source] StoreError),
    #[error("Component {0} initialization failed, internal error was: {1}")]
    ComponentInitError(String, #[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("The feature has not yet been implemented")]
    Unimplemented,
    #[error("Unknown error, internal error was: {0}")]
    Unknown(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Something went wrong with the internal runtime, perhaps stack corruption")]
    BroadcastError,
    #[error("Failed to insert events even after {0} attempts, the events that couldn't be persisted was: {:#?}")]
    InsertFailed(u64, Vec<Event<D, M>>),
    #[error("An attempt was made to build an aggregate from zero events")]
    BuildAggregateFromZeroEvents,
}
