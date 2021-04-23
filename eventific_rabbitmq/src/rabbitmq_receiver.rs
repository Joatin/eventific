use futures::{Future, Stream, StreamExt};
use std::process;
use std::sync::{Arc, RwLock};
use eventific::Uuid;
use eventific::{Eventific};
use eventific::store::Store;
use std::fmt::Debug;
use std::error::Error;
use strum::IntoEnumIterator;
use lapin::{Connection, ConnectionProperties, ExchangeKind};
use tokio_amqp::LapinTokioExt;
use lapin::options::{QueueDeclareOptions, BasicConsumeOptions, BasicAckOptions, ExchangeDeclareOptions, ExchangeBindOptions, QueueBindOptions};
use lapin::types::FieldTable;
use eventific::notification::Receiver;
use tokio::sync::broadcast::{Sender as TokioSender};

#[derive(Debug)]
pub struct RabbitMqReceiver {
    amqp_address: String,
    service_group_name: String
}

impl RabbitMqReceiver {
    pub fn new(amqp_address: &str, service_group_name: &str) -> Self {
        Self {
            amqp_address: amqp_address.to_owned(),
            service_group_name: service_group_name.to_owned()
        }
    }

    fn get_queue_name(&self, service_name: &str) -> String {
        format!("{}_{}_listener", service_name, self.service_group_name)
    }
}

#[eventific::async_trait]
impl<
    St: Store<EventData = D, MetaData = M>,
    S: 'static + Send + Debug + Default,
    D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator + AsRef<str>,
    M: 'static + Clone + Send + Sync + Debug,
> Receiver<St, S, D, M> for RabbitMqReceiver {

    #[tracing::instrument]
    async fn init(&mut self, eventific: &Eventific<St, S, D, M>, sender: TokioSender<Uuid>) -> Result<(), Box<dyn Error + Send + Sync>> {
        let conn = Connection::connect(
            &self.amqp_address,
            ConnectionProperties::default().with_tokio(),
        )
            .await?;

        let channel = conn.create_channel().await?;

        let queue_name = &self.get_queue_name(eventific.service_name());

        let _exchange = channel.exchange_declare(eventific.service_name(), ExchangeKind::Fanout, ExchangeDeclareOptions::default(), FieldTable::default()).await?;
        let _queue = channel.queue_declare(queue_name, QueueDeclareOptions::default(), FieldTable::default()).await?;
        let _bind = channel.queue_bind(queue_name, eventific.service_name(), "", QueueBindOptions::default(), FieldTable::default()).await?;

        let mut consumer = channel
            .basic_consume(
                queue_name,
                eventific.service_name(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        tokio::spawn(async move {
            while let Some(delivery) = consumer.next().await {
                let (_, delivery) = delivery.expect("error in consumer");

                let id = Uuid::from_slice(&delivery.data).expect("Invalid uuid");
                let _res = sender.send(id);

                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("ack");
            }
        });

        Ok(())
    }

    fn name(&self) -> &str {
        "RabbitMQ Receiver 🐰🦻"
    }
}
