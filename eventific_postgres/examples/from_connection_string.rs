use eventific::EventStore;
use eventific_postgres::PostgresStorage;

#[derive(serde::Serialize, serde::Deserialize)]
enum Payload {
    #[allow(dead_code)]
    Created,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let postgres_storage = PostgresStorage::from_connection_string(
        "host=localhost user=admin password=password",
        "example",
    )
    .await?;

    let event_store = EventStore::<Payload>::builder().build(postgres_storage);

    let _total_events = event_store.total_events().await?;

    Ok(())
}
