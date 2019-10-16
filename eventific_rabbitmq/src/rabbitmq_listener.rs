use eventific::notification::{Listener, NotificationError};
use slog::Logger;
use futures::{Future, Stream};
use uuid::Uuid;
use lapin_futures::{Client, ConnectionProperties, Consumer, Channel, Queue};
use std::sync::{RwLock, Arc};
use lapin_futures::options::QueueDeclareOptions;
use lapin_futures::options::QueueBindOptions;
use lapin_futures::options::BasicConsumeOptions;
use lapin_futures::types::FieldTable;
use lapin_futures::message::Delivery;

pub struct RabbitMqListener {
    amqp_address: String,
    queue_postfix: String,
    logger: Option<Logger>,
    client: Option<Client>,
    queue_name: Option<String>
}

impl RabbitMqListener {
    pub fn new(amqp_address: &str, queue_postfix: &str) -> Self {
        Self {
            amqp_address: amqp_address.to_owned(),
            queue_postfix: queue_postfix.to_owned(),
            logger: None,
            client: None,
            queue_name: None
        }
    }

    fn create_channel(logger: &Logger, client: &Client) -> impl Future<Item=(Logger, Channel), Error=NotificationError> {
        let log = logger.clone();
        let err_log = logger.clone();
        info!(log, "Establishing a new channel to 🐰 RabbitMq");
        client.create_channel()
            .map_err(move |err| {
                error!(err_log, "Failed to establish a channel to 🐰 RabbitMQ"; "error" => format!("{}", err));
                NotificationError::Unknown(format_err!("{}", err))
            })
            .map(move |c| (log, c))
    }

    fn create_queue((logger, channel): (Logger, Channel), queue_name: &str) -> impl Future<Item=(Logger, Channel, Queue), Error=NotificationError> {
        let options = QueueDeclareOptions {
            durable: false,
            ..Default::default()
        };

        let err_log = logger.clone();
        info!(logger, "Declaring queue {}", queue_name);

        channel.queue_declare(&queue_name, options, FieldTable::default())
            .map_err(move |err| {
                error!(err_log, "Failed to create queue in 🐰 RabbitMQ"; "error" => format!("{}", err));
                NotificationError::Unknown(format_err!("{}", err))
            })
            .and_then(move |queue| {
                Ok((logger, channel, queue))
            })
    }

    fn consume_queue((logger, channel, queue): (Logger, Channel, Queue)) -> impl Stream<Item=Delivery, Error=NotificationError> {
        info!(logger, "Starting to tail queue");
        let err_log = logger.clone();

        let options = BasicConsumeOptions {
            no_ack: true,
            no_local: true,
            nowait: false,
            exclusive: false
        };

        channel.basic_consume(&queue, "eventific", options, FieldTable::default())
            .map_err(move |err| {
                error!(err_log, "Failed to tail rabbit queue"; "error" => format!("{}", err));
                NotificationError::Unknown(format_err!("{}", err))
            })
            .and_then(move |consumer| {
                info!(logger, "Successfully started listening to queue");
                Ok(consumer.map_err(|err| NotificationError::FailedToListen(format_err!("{}", err))))
            })
            .into_stream()
            .flatten()
    }
}

impl Listener for RabbitMqListener {
    fn init(&mut self, logger: &Logger, service_name: &str) -> Box<dyn Future<Item=(), Error=NotificationError> + Send> {
        self.logger.replace(logger.new(o!("listener" => "rabbitmq")));
        self.queue_name.replace(format!("{}-{}", service_name, self.queue_postfix));

        let log = self.logger.as_ref().unwrap();

        info!(log, "Initializing 🐰 RabbitMQ listener");

        match Client::connect(&self.amqp_address, ConnectionProperties::default()).wait() {
            Ok(client) => {
                info!(log, "Succesfully initialized 🐰 RabbitMQ listener");
                self.client.replace(client);
                Box::new(futures::finished(()))
            },
            Err(err) => {
                error!(log, "Failed to initialize 🐰 RabbitMQ listener");
                Box::new(futures::failed(NotificationError::Unknown(format_err!("{}", err))))
            },
        }
    }

    fn listen(&self) -> Box<dyn Stream<Item=Uuid, Error=NotificationError> + Send> {
        let client = self.client.as_ref().expect("The listener has to be initialized");
        let logger = self.logger.as_ref().unwrap().clone();
        let queue_name = self.queue_name.clone().expect("The listener has to be initialized");

        info!(logger, "Starting subscription to rabbit queue");

        Box::new(
            Self::create_channel(&logger, &client)
                .and_then(move |p| Self::create_queue(p, &queue_name))
                .map(Self::consume_queue)
                .into_stream()
                .flatten()
                .and_then(move |delivery| {
                    match Uuid::from_slice(&delivery.data) {
                        Ok(uuid) => {
                            info!(logger, "Successfully parsed uuid"; "uuid" => format!("{}", uuid));
                            Ok(uuid)
                        },
                        Err(err) => {
                            warn!(logger, "Failed to parse UUID"; "error" => format!("{}", err));
                            Ok(Uuid::nil())
                        }
                    }
                })
                .filter(|uuid| {
                    !uuid.is_nil()
                })
        )
    }
}
