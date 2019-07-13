use crate::store::StoreError;
use std::fmt::Debug;
use crate::event::Event;

#[derive(Debug, failure::Fail)]
pub enum EventificError<D: 'static + Send + Sync + Debug> {
    #[fail(display = "The events seems to be missing a event, or they appear in the wrong order, the events was {:?}", _0)]
    InconsistentEventChain(Vec<Event<D>>),
    #[fail(display = "Failed while setting up store, internal error was: {}", _0)]
    StoreInitError(#[fail(cause)] StoreError<D>),
    #[fail(display = "Store failed, internal error was: {}", _0)]
    StoreError(#[fail(cause)] StoreError<D>),
    #[fail(display = "The feature has not yet been implemented")]
    Unimplemented,
    #[fail(display = "Unknown error, internal error was: {}", _0)]
    Unknown(#[fail(cause)] failure::Error),
}
