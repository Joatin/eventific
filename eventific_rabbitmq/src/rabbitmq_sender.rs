use eventific::notification::{NotificationError, Sender};
use futures::Future;
use lapin_futures::options::BasicPublishOptions;
use lapin_futures::options::ExchangeDeclareOptions;
use lapin_futures::types::FieldTable;
use lapin_futures::{BasicProperties, Channel, Client, ConnectionProperties};
use slog::Logger;
use std::process;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct RabbitMqSender {
    amqp_address: String,
    exchange_name: Option<String>,
    logger: Option<Logger>,
    client: Option<Client>,
}

impl RabbitMqSender {
    pub fn new(amqp_address: &str) -> Self {
        Self {
            amqp_address: amqp_address.to_owned(),
            logger: None,
            exchange_name: None,
            client: None,
        }
    }
}

impl Sender for RabbitMqSender {
    fn init(
        &mut self,
        logger: &Logger,
        service_name: &str,
    ) -> Box<dyn Future<Item = (), Error = NotificationError> + Send> {
        self.logger.replace(logger.new(o!("sender" => "rabbitmq")));
        let exchange_name = service_name.to_owned();
        self.exchange_name.replace(exchange_name.to_owned());

        let log = logger.clone();
        let log2 = logger.clone();

        info!(log, "Initializing new 🐰 RabbitMq Sender!");

        match Client::connect(&self.amqp_address, ConnectionProperties::default()).wait() {
            Ok(client) => {
                client.on_error(Box::new(|| {
                    eprintln!("Rabbitmq Error");
                    eprintln!("Shutting down eventific...");
                    process::exit(1);
                }));
                info!(log, "Successfully initialized new 🐰 RabbitMq Sender!");
                self.client.replace(client);
                Box::new(futures::finished(()))
            }
            Err(err) => {
                error!(log, "Failed to initialize 🐰 RabbitMQ sender");
                Box::new(futures::failed(NotificationError::Unknown(format_err!(
                    "{}", err
                ))))
            }
        }
    }

    fn send(
        &self,
        aggregate_id: Uuid,
    ) -> Box<dyn Future<Item = (), Error = NotificationError> + Send> {
        let client = self
            .client
            .as_ref()
            .expect("The listener has to be initialized");
        let logger = self.logger.as_ref().unwrap().clone();
        let err_logger = logger.clone();
        let exchange_name = self
            .exchange_name
            .clone()
            .expect("The listener has to be initialized");

        info!(logger, "Sending notification to rabbit exchange"; "uuid" => format!("{}", &aggregate_id));

        Box::new(client.create_channel()
            .map_err(move |err| {
                error!(err_logger, "Failed to open channel to rabbit"; "error" => format!("{}", err));
                NotificationError::FailedToSend(format_err!("{}", err))
            })
            .and_then(move |channel| {
                let payload = aggregate_id.as_bytes().to_vec();
                let options = BasicPublishOptions::default();
                let properties = BasicProperties::default();
                let err_log = logger.clone();
                channel.basic_publish(&exchange_name, "", payload, options, properties)
                    .map_err(move |err| {
                        error!(err_log, "Failed to send message to rabbit exchange"; "error" => format!("{}", err));
                        NotificationError::FailedToSend(format_err!("{}", err))
                    })
                    .map(move |_| {
                        info!(logger, "Successfully sent message to rabbit exchange");
                        ()
                    })
            }))
    }
}
