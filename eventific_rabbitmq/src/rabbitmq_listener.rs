use eventific::notification::{Listener, NotificationError};
use slog::Logger;
use futures::{Future, Stream};
use uuid::Uuid;
use lapin_futures::{Client, ConnectionProperties, Consumer};
use std::sync::{RwLock, Arc};
use lapin_futures::options::QueueDeclareOptions;
use lapin_futures::options::QueueBindOptions;
use lapin_futures::options::BasicConsumeOptions;
use lapin_futures::types::FieldTable;

pub struct RabbitMqListener {
    amqp_address: String,
    queue_postfix: String,
    logger: Option<Logger>,
    consumer: Arc<RwLock<Option<Consumer>>>,
}

impl RabbitMqListener {
    pub fn new(amqp_address: &str, queue_postfix: &str) -> Self {
        Self {
            amqp_address: amqp_address.to_owned(),
            queue_postfix: queue_postfix.to_owned(),
            logger: None,
            consumer: Arc::new(RwLock::new(None))
        }
    }
}

impl Listener for RabbitMqListener {
    fn init(&mut self, logger: &Logger, service_name: &str) -> Box<dyn Future<Item=(), Error=NotificationError> + Send> {
        self.logger.replace(logger.clone());
        let queue_name = format!("{}-{}", service_name, self.queue_postfix);
        let exchange_name = service_name.to_owned();
        let consumer_arc = Arc::clone(&self.consumer);

        Box::new(Client::connect(&self.amqp_address, ConnectionProperties::default())
            .map_err(|err| NotificationError::Unknown(format_err!("{}", err)))
            .and_then(|client| {
                client.create_channel().map_err(|err| NotificationError::Unknown(format_err!("{}", err)))
            }).and_then(move |channel| {

            let options = QueueDeclareOptions {
                durable: false,
                ..Default::default()
            };

            channel.queue_declare(&queue_name, options, FieldTable::default())
                .map_err(|err| NotificationError::Unknown(format_err!("{}", err)))
                .and_then(move |queue| {
                    channel.queue_bind(&queue_name, &exchange_name, "", QueueBindOptions::default(), FieldTable::default())
                        .map_err(|err| NotificationError::Unknown(format_err!("{}", err)))
                        .and_then(move |_| {
                            channel.basic_consume(&queue, "eventific", BasicConsumeOptions::default(), FieldTable::default())
                                .map_err(|err| NotificationError::Unknown(format_err!("{}", err)))
                                .and_then(move |consumer| {
                                    let mut lock = consumer_arc.write().unwrap();
                                    lock.replace(consumer);
                                    Ok(())
                                })
                    })
                })
            }))
    }

    fn listen(&self) -> Box<dyn Stream<Item=Uuid, Error=NotificationError> + Send> {
        let lock = self.consumer.read().unwrap();
        let consumer = lock.as_ref().unwrap();

        Box::new(consumer.clone()
            .map_err(|err| NotificationError::FailedToListen(format_err!("{}", err)))
            .and_then(|delivery| {
            Uuid::from_slice(&delivery.data)
                .map_err(|err| NotificationError::FailedToListen(format_err!("{}", err)))
            })
            .map_err(|err| NotificationError::FailedToListen(format_err!("{}", err)))
        )
    }
}
