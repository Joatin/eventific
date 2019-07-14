use crate::event::Event;
use std::fmt::Debug;

#[derive(Debug, failure::Fail)]
pub enum NotificationError {
    #[fail(display = "Failed to send notification, internal error was: {}", _0)]
    FailedToSend(#[fail(cause)] failure::Error),
    #[fail(display = "Failed to listen for notifications, internal error was: {}", _0)]
    FailedToListen(#[fail(cause)] failure::Error),
    #[fail(display = "Unknown error, internal error was: {}", _0)]
    Unknown(#[fail(cause)] failure::Error),
}
