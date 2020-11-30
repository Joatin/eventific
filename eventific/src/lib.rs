//! Eventific is a utility that makes implementing Event Sourcing with CQRS easier

#![warn(missing_docs)]

extern crate chrono;
extern crate futures;
extern crate uuid;
#[macro_use]
extern crate slog;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod aggregate;
mod component;
mod event;
mod eventific;
pub mod store;
pub mod test;

pub use self::aggregate::Aggregate;
pub use self::aggregate::StateBuilder;
pub use self::component::Component;
pub use self::event::Event;
pub use self::eventific::*;
