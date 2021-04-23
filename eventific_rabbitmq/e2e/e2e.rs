use eventific::{EventificBuilder, CreateAggregateParams};
use eventific_rabbitmq::{RabbitMqReceiver, RabbitMqSender};
use eventific::store::MemoryStore;
use eventific::test::TestEventData;
use eventific::test::test_state_builder;
use eventific::Uuid;
use futures::StreamExt;
use std::time::Duration;
use tokio::time::timeout;
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};

#[tokio::test(flavor = "multi_thread")]
async fn it_should_be_working() -> Result<(), Box<dyn std::error::Error>> {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    println!("Hello");

    let eventific = EventificBuilder::new()
        .receiver(RabbitMqReceiver::new("amqp://guest:guest@localhost:5672/%2f", "test"))
        .sender(RabbitMqSender::new("amqp://guest:guest@localhost:5672/%2f"))
        .build(
            "test_service",
            test_state_builder,
            MemoryStore::new()
        )
        .await?;

    let eventific2 = eventific.clone();

    let handle = tokio::spawn(async move {
        let mut stream = eventific.updated_aggregates().await.unwrap();
        stream.next().await.unwrap();
    });

    tokio::time::sleep(Duration::from_secs(1)).await;

    eventific2.create_aggregate(CreateAggregateParams {
        aggregate_id: Uuid::new_v4(),
        events: vec![
            TestEventData::Event1
        ],
        metadata: None
    }).await?;

    if let Err(_) = timeout(Duration::from_secs(5), handle).await {
        return Err("did not receive any response".into());
    }

    Ok(())
}
