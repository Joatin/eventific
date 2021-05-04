//! Eventific is a utility that makes implementing Event Sourcing with CQRS easier

#![warn(missing_docs)]

#[macro_use]
extern crate tracing;

mod aggregate;
mod component;
mod event;
mod eventific;
pub mod store;
pub mod test;
pub mod notification;

pub use async_trait::async_trait;
pub use uuid::*;
pub use chrono::*;

pub use self::aggregate::Aggregate;
pub use self::aggregate::StateBuilder;
pub use self::component::Component;
pub use self::event::Event;
pub use self::eventific::*;
