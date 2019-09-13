#[macro_use]
extern crate failure;

mod rabbitmq_listener;
mod rabbitmq_sender;

pub use self::rabbitmq_sender::RabbitMqSender;
pub use self::rabbitmq_listener::RabbitMqListener;
