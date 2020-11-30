#![warn(missing_docs)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate slog;

mod postgres_store;

pub use self::postgres_store::PostgresStore;
