use crate::store::StoreError;
use std::fmt::Debug;
use crate::event::{Event, EventData};

#[derive(Debug, failure::Fail)]
pub enum EventificError<D: EventData, M: 'static + Send + Sync + Debug> {
    #[fail(display = "Apply event callback returned validation failure, internal error was {}", _0)]
    ValidationError(#[fail(cause)] failure::Error),
    #[fail(display = "The events seems to be missing a event, or they appear in the wrong order, the event was {:?}", _0)]
    InconsistentEventChain(Event<D, M>),
    #[fail(display = "Failed while setting up store, internal error was: {}", _0)]
    StoreInitError(#[fail(cause)] StoreError<D, M>),
    #[fail(display = "Failed while setting up eventific, internal error was: {}", _0)]
    InitError(#[fail(cause)] failure::Error),
    #[fail(display = "Store failed, internal error was: {}", _0)]
    StoreError(#[fail(cause)] StoreError<D, M>),
    #[fail(display = "Component {} initialization failed, internal error was: {}", _0, _1)]
    ComponentInitError(&'static str, #[fail(cause)] failure::Error),
    #[fail(display = "The feature has not yet been implemented")]
    Unimplemented,
    #[fail(display = "Unknown error, internal error was: {}", _0)]
    Unknown(#[fail(cause)] failure::Error)
}
