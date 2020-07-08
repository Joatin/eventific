use eventific::notification::{Listener, NotificationError};
use slog::Logger;
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use uuid::Uuid;
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::ClientConfig;
use futures::{FutureExt, TryStreamExt};
use futures::stream::StreamExt;
use std::time::Duration;
use rdkafka::message::Message;
use std::str::from_utf8;


pub struct KafkaListener {
    consumer: Option<StreamConsumer>,
    brokers: String
}

impl KafkaListener {

    pub fn new(brokers: &str) -> Self {
        Self {
            consumer: None,
            brokers: brokers.to_string()
        }
    }

}

impl Listener for KafkaListener {
    fn init<'a>(&'a mut self, logger: &'a Logger, service_name: &'a str) -> BoxFuture<'a, Result<(), NotificationError>> {
        async move {
            let consumer: StreamConsumer = ClientConfig::new()
                .set("bootstrap.servers", &self.brokers)
                .set("session.timeout.ms", "6000")
                .set("enable.auto.commit", "false")
                .set("group.id", &service_name)
                .create()
                .map_err(|err| NotificationError::Unknown(format_err!("{}", err)))?;


            consumer.subscribe(&[&service_name]).unwrap();

            info!(logger, "Successfully established connection with Kafka"; "listener" => "kafka");

            self.consumer.replace(consumer);

            Ok(())
        }.boxed()
    }

    fn listen<'a>(&'a self, logger: &'a Logger) -> BoxFuture<'a, Result<BoxStream<'a, Result<Uuid, NotificationError>>, NotificationError>> {
        async move {
            let consumer = self.consumer.as_ref().expect("Listener has to be initialized");
            let stream = consumer.start_with(Duration::from_millis(10), false);

            info!(logger, "Started tailing Kafka messages"; "listener" => "kafka");

            let mapped_stream = stream
                .map_err(|err| NotificationError::FailedToListen(format_err!("{}", err)))
                .and_then(|message| async move {
                    let payload = message.payload().ok_or(NotificationError::FailedToListen(format_err!("Missing payload")))?;
                    let raw_uuid = from_utf8(payload).map_err(|_| NotificationError::FailedToListen(format_err!("Invalid payload encoding")))?;
                    let aggregate_id = Uuid::parse_str(raw_uuid).map_err(|_| NotificationError::FailedToListen(format_err!("The payload was not a valid Uuid")))?;
                    Ok(aggregate_id)
                });

            let boxed_stream: BoxStream<_> = mapped_stream.boxed();

            Ok(boxed_stream)
        }.boxed()
    }
}
