//! Eventific is a utility that makes implementing Event Sourcing with CQRS easier

#![warn(missing_docs)]

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
mod event;
pub mod store;
mod aggregate;
pub mod test;
mod component;

pub use self::eventific::Eventific;
pub use self::eventific::EventificBuilder;
pub use self::eventific::EventificError;
pub use self::aggregate::StateBuilder;
pub use self::aggregate::Aggregate;
pub use self::event::EventData;
pub use self::event::Event;
pub use self::component::Component;
