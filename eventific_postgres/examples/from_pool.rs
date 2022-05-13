use eventific_postgres::PostgresStorage;
use eventific::EventStore;
use std::str::FromStr;
use bb8_postgres::PostgresConnectionManager;
use bb8_postgres::bb8::Pool;

#[derive(serde::Serialize, serde::Deserialize)]
enum Payload {
    #[allow(dead_code)]
    Created
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {


    let config = tokio_postgres::config::Config::from_str("host=localhost user=admin password=password")?;
    let pg_mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);

    let pool = Pool::builder().build(pg_mgr).await?;

    let postgres_storage = PostgresStorage::new(pool, "example").await?;

    let event_store = EventStore::<Payload>::builder().build(postgres_storage);

    let _total_events = event_store.total_events().await?;

    Ok(())
}
