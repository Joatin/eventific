#![warn(missing_docs)]

#[macro_use]
extern crate slog;

mod postgres_store;
mod postgres_store_error;

pub use self::postgres_store::PostgresStore;
