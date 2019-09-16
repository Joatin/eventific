use eventific::notification::{Sender, NotificationError};
use slog::Logger;
use futures::Future;
use uuid::Uuid;
use std::sync::{RwLock, Arc};
use lapin_futures::{Channel, ConnectionProperties, Client, BasicProperties};
use lapin_futures::options::ExchangeDeclareOptions;
use lapin_futures::options::BasicPublishOptions;
use lapin_futures::types::FieldTable;

pub struct RabbitMqSender {
    amqp_address: String,
    exchange_name: Option<String>,
    logger: Option<Logger>,
    channel: Arc<RwLock<Option<Channel>>>,
}

impl RabbitMqSender {
    pub fn new(amqp_address: &str) -> Self {
        Self {
            amqp_address: amqp_address.to_owned(),
            logger: None,
            exchange_name: None,
            channel: Arc::new(RwLock::new(None))
        }
    }
}

impl Sender for RabbitMqSender {
    fn init(&mut self, logger: &Logger, service_name: &str) -> Box<dyn Future<Item=(), Error=NotificationError> + Send> {
        self.logger.replace(logger.clone());
        let exchange_name = service_name.to_owned();
        self.exchange_name.replace(exchange_name.to_owned());
        let channel_arc = Arc::clone(&self.channel);

        let log = logger.clone();
        let log2 = logger.clone();

        info!(log, "Setting up new RabbitMq Notification Sender!");

        Box::new(Client::connect(&self.amqp_address, ConnectionProperties::default())
            .map_err(|err| NotificationError::Unknown(format_err!("{}", err)))
            .and_then(move |client| {
                info!(log, "Successfully connected to rabbit, opening a fresh channel");
                client.create_channel().map_err(|err| NotificationError::Unknown(format_err!("{}", err)))
            }).and_then(move |channel| {

            let options = ExchangeDeclareOptions {
                ..Default::default()
            };

            info!(log2, "Successfully connected to rabbit, opening a fresh channel");
            channel.exchange_declare(&exchange_name, "", options, FieldTable::default())
                .map_err(|err| NotificationError::Unknown(format_err!("{}", err)))
                .and_then(move |_| {
                    info!(log2, "RabbitMqSender setup complete");
                    let mut lock = channel_arc.write().unwrap();
                    lock.replace(channel);
                    Ok(())
                })
        }))
    }

    fn send(&self, aggregate_id: Uuid) -> Box<dyn Future<Item=(), Error=NotificationError> + Send> {
        let channel = {
            let lock = self.channel.read().unwrap();
            lock.as_ref().unwrap()
        };

        let payload = aggregate_id.as_bytes().to_vec();

        let options = BasicPublishOptions::default();

        let properties = BasicProperties::default();

        Box::new(channel.basic_publish(self.exchange_name.as_ref().unwrap(), "", payload, options, properties)
            .map_err(|err| NotificationError::FailedToSend(format_err!("{}", err)))
            .map(|_| ())
        )
    }
}
