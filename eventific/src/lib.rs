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
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

mod eventific;


pub mod event;
pub mod store;
pub mod aggregate;
pub mod notification;
pub mod test;

pub use self::eventific::Eventific;
pub use self::eventific::EventificBuilder;
pub use self::eventific::EventificError;
