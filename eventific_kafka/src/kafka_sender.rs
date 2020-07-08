use eventific::notification::{Sender, NotificationError};
use uuid::Uuid;
use futures::future::BoxFuture;
use slog::Logger;
use futures::FutureExt;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;

pub struct KafkaSender {
    brokers: String,
    topic: Option<String>,
    producer: Option<FutureProducer>
}

impl KafkaSender {

    pub fn new(brokers: &str) -> Self {
        Self {
            brokers: brokers.to_string(),
            topic: None,
            producer: None
        }
    }
}

impl Sender for KafkaSender {
    fn init<'a>(&'a mut self, logger: &'a Logger, service_name: &'a str) -> BoxFuture<'a, Result<(), NotificationError>> {
        async move {
            let producer: FutureProducer = ClientConfig::new()
                .set("bootstrap.servers", &self.brokers)
                .set("message.timeout.ms", "100")
                .create()
                .map_err(|err| NotificationError::Unknown(format_err!("{}", err)))?;

            info!(logger, "Successfully established connection with Kafka"; "sender" => "kafka");

            self.producer.replace(producer);
            self.topic.replace(service_name.to_string());
            Ok(())
        }.boxed()
    }

    fn send<'a>(&'a self, logger: &'a Logger, aggregate_id: Uuid) -> BoxFuture<'a, Result<(), NotificationError>> {
        async move {
            let producer = self.producer.as_ref().expect("Sender not initialized");
            let topic = self.topic.as_ref().expect("Sender not initialized");
            let payload = aggregate_id.to_string();

            let message: FutureRecord<String, String> = FutureRecord::to(&topic)
                .payload(&payload)
                .timestamp(now());

            let (_partition, _offset) = producer.send(message, 0)
                .await
                .map_err(|err| NotificationError::Unknown(format_err!("{}", err)))?
                .map_err(|(err, _)| NotificationError::Unknown(format_err!("{}", err)))?;

            Ok(())
        }.boxed()
    }
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .try_into()
        .unwrap()
}
