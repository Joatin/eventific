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

mod eventific;

#[cfg(feature = "with_grpc")]
pub mod grpc;

pub mod event;
pub mod store;
pub mod aggregate;
pub mod notification;

#[cfg(test)]
pub mod test;

pub use self::eventific::Eventific;
pub use self::eventific::EventificBuilder;
