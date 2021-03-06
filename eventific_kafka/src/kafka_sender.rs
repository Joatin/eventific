use eventific::notification::{NotificationError, Sender};
use futures::future::BoxFuture;
use futures::FutureExt;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use slog::Logger;
use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct KafkaSender {
    brokers: String,
    topic: Option<String>,
    producer: Option<FutureProducer>,
}

impl KafkaSender {
    pub fn new(brokers: &str) -> Self {
        Self {
            brokers: brokers.to_string(),
            topic: None,
            producer: None,
        }
    }
}

impl Sender for KafkaSender {
    fn init<'a>(
        &'a mut self,
        logger: &'a Logger,
        service_name: &'a str,
    ) -> BoxFuture<'a, Result<(), NotificationError>> {
        async move {
            let producer: FutureProducer = ClientConfig::new()
                .set("bootstrap.servers", &self.brokers)
                .set("message.timeout.ms", "5000")
                .create()
                .map_err(|err| NotificationError::Unknown(format_err!("{}", err)))?;

            info!(logger, "Successfully established connection with Kafka"; "sender" => "kafka");

            self.producer.replace(producer);
            self.topic.replace(service_name.to_string());
            Ok(())
        }
        .boxed()
    }

    fn send<'a>(
        &'a self,
        logger: &'a Logger,
        aggregate_id: Uuid,
    ) -> BoxFuture<'a, Result<(), NotificationError>> {
        async move {
            let producer = self.producer.as_ref().expect("Sender not initialized");
            let topic = self.topic.as_ref().expect("Sender not initialized");
            let payload = aggregate_id.to_string();

            let message: FutureRecord<String, String> =
                FutureRecord::to(&topic).payload(&payload).timestamp(now());

            let (_partition, _offset) = producer
                .send(message, 5000)
                .await
                .map_err(|err| NotificationError::Unknown(format_err!("{}", err)))?
                .map_err(|(err, _)| NotificationError::Unknown(format_err!("{}", err)))?;

            Ok(())
        }
        .boxed()
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
