
#[macro_use]
extern crate failure;
#[macro_use]
extern crate slog;

mod sqs_listener;

pub use self::sqs_listener::SqsListener;
