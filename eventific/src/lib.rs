extern crate uuid;
extern crate chrono;
#[macro_use]
extern crate failure;
extern crate futures;
#[macro_use]
extern crate slog;

mod eventific;

pub mod event;
pub mod store;
pub mod aggregate;
pub mod notification;

pub use self::eventific::Eventific;
pub use self::eventific::EventificBuilder;
