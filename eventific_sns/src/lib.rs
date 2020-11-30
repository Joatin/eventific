#[macro_use]
extern crate slog;
#[macro_use]
extern crate failure;

mod sns_sender;

pub use self::sns_sender::SnsSender;
