use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use eventific::EventStore;
use eventific_postgres::PostgresStorage;
use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize)]
enum Payload {
    #[allow(dead_code)]
    Created,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config =
        tokio_postgres::config::Config::from_str("host=localhost user=admin password=password")?;
    let pg_mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);

    let pool = Pool::builder().build(pg_mgr).await?;

    let postgres_storage = PostgresStorage::new(pool, "example").await?;

    let event_store = EventStore::<Payload>::builder().build(postgres_storage);

    let _total_events = event_store.total_events().await?;

    Ok(())
}
