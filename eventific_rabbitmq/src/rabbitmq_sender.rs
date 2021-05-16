use futures::{Future, TryStreamExt, FutureExt, StreamExt};
use std::process;
use std::sync::{Arc, RwLock};
use eventific::Uuid;
use eventific::{Component, Eventific};
use eventific::store::Store;
use std::fmt::Debug;
use strum::IntoEnumIterator;
use std::error::Error;
use lapin::{Connection, ConnectionProperties, ExchangeKind, BasicProperties};
use lapin::options::{ExchangeDeclareOptions, BasicPublishOptions};
use tokio_amqp::LapinTokioExt;
use lapin::types::FieldTable;
use tracing::Instrument;
use eventific::notification::Sender;
use tokio::sync::broadcast::{ Receiver as TokioReceiver };

#[derive(Debug)]
pub struct RabbitMqSender {
    amqp_address: String
}

impl RabbitMqSender {
    pub fn new(amqp_address: &str) -> Self {
        Self {
            amqp_address: amqp_address.to_owned(),
        }
    }
}

#[eventific::async_trait]
impl<
    St: Store<EventData = D, MetaData = M>,
    S: 'static + Send + Sync + Debug + Default,
    D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator + AsRef<str>,
    M: 'static + Clone + Send + Sync + Debug,
> Sender<St, S, D, M> for RabbitMqSender {
    #[tracing::instrument]
    async fn init(&mut self, eventific: &Eventific<St, S, D, M>, mut receiver: TokioReceiver<Uuid>) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service_name = eventific.service_name().to_owned();
        let instance_id = eventific.instance_id().to_owned();

        let conn = Connection::connect(
            &self.amqp_address,
            ConnectionProperties::default().with_tokio(),
        )
            .await?;

        let channel = conn.create_channel().await?;

        let _exchange = channel.exchange_declare(&service_name, ExchangeKind::Fanout, ExchangeDeclareOptions::default(), FieldTable::default()).await?;

        tokio::spawn(async move {
            loop {
                match receiver.recv().await {
                    Ok(id) => {
                        channel.clone().basic_publish(
                            &service_name,
                            "",
                            BasicPublishOptions::default(),
                            Uuid::as_bytes(&id).to_vec(),
                            BasicProperties::default()
                        ).await.unwrap();
                    },
                    Err(_) => {
                        tracing::error!("Can't keep up with messages")
                    }
                }
            }
        });

        Ok(())
    }

    fn name(&self) -> &str {
        "RabbitMQ Sender 🐰🗣"
    }
}
