use crate::event::Event;
use std::fmt::Debug;

#[derive(Debug, failure::Fail)]
pub enum StoreError<D: 'static + Send + Sync + Debug> {
    #[fail(display = "The event already exists, this is most likely due to another servicing pushing events before us, event: \n{:#?}", _0)]
    EventAlreadyExists(Event<D>),
    #[fail(display = "Invalid credentials to underlying store, internal error was: {}", _0)]
    CredentialsError(#[fail(cause)] failure::Error),
    #[fail(display = "Failed to connect to store, internal error was: {}", _0)]
    ConnectError(#[fail(cause)] failure::Error),
    #[fail(display = "Action is unsupported, description: {}", _0)]
    Unsupported(String),
    #[fail(display = "Unknown error, internal error was: {}", _0)]
    Unknown(#[fail(cause)] failure::Error),
}
