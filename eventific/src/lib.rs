extern crate uuid;
extern crate chrono;
#[macro_use]
extern crate failure;
extern crate futures;
#[macro_use]
extern crate slog;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "playground")]
extern crate hyper;
#[cfg(feature = "playground")]
extern crate tokio;
#[cfg(feature = "playground")]
#[macro_use]
extern crate rust_embed;

mod eventific;

#[cfg(feature = "playground")]
mod playground;

pub mod event;
pub mod store;
pub mod aggregate;
pub mod notification;

pub use self::eventific::Eventific;
pub use self::eventific::EventificBuilder;
