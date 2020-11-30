#[macro_use]
extern crate failure;
#[macro_use]
extern crate slog;

mod rabbitmq_listener;
mod rabbitmq_sender;

pub use self::rabbitmq_listener::RabbitMqListener;
pub use self::rabbitmq_sender::RabbitMqSender;
