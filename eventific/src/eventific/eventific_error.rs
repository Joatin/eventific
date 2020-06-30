use crate::store::StoreError;
use std::fmt::Debug;
use crate::event::{Event, EventData};
use crate::notification::NotificationError;

#[derive(Debug, failure::Fail)]
pub enum EventificError<D: EventData> {
    #[fail(display = "Apply event callback returned validation failure, internal error was {}", _0)]
    ValidationError(#[fail(cause)] failure::Error),
    #[fail(display = "The events seems to be missing a event, or they appear in the wrong order, the event was {:?}", _0)]
    InconsistentEventChain(Event<D>),
    #[fail(display = "Failed while setting up store, internal error was: {}", _0)]
    StoreInitError(#[fail(cause)] StoreError<D>),
    #[fail(display = "Failed while setting up eventific, internal error was: {}", _0)]
    InitError(#[fail(cause)] failure::Error),
    #[fail(display = "Store failed, internal error was: {}", _0)]
    StoreError(#[fail(cause)] StoreError<D>),
    #[fail(display = "Notification sender failed, internal error was: {}", _0)]
    SendNotificationError(#[fail(cause)] NotificationError),
    #[fail(display = "Notification sender initialization failed, internal error was: {}", _0)]
    SendNotificationInitError(#[fail(cause)] NotificationError),
    #[fail(display = "Notification listener failed, internal error was: {}", _0)]
    ListenNotificationError(#[fail(cause)] NotificationError),
    #[fail(display = "Notification listener initialization failed, internal error was: {}", _0)]
    ListenNotificationInitError(#[fail(cause)] NotificationError),
    #[fail(display = "The feature has not yet been implemented")]
    Unimplemented,
    #[fail(display = "Unknown error, internal error was: {}", _0)]
    Unknown(#[fail(cause)] failure::Error),
}
