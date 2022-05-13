use eventific::{Storage, SaveEventsResult};
use tokio_postgres::{NoTls, Socket};
use bb8_postgres::bb8::{Pool};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::tls::{MakeTlsConnect, TlsConnect};
use std::str::FromStr;
use serde::Serialize;
use serde::de::DeserializeOwned;
use futures::{StreamExt, TryStreamExt};
use futures::stream;
use futures::stream::BoxStream;

pub struct PostgresStorage<Tls>
    where
        Tls: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <Tls as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <Tls as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<Tls as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pool: Pool<PostgresConnectionManager<Tls>>,
    table_name: String
}

impl<Tls> PostgresStorage<Tls>
    where
        Tls: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <Tls as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <Tls as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<Tls as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{

    pub async fn new(pool: Pool<PostgresConnectionManager<Tls>>, table_name: &str) -> anyhow::Result<Self> {
        Self::create_table(&pool, table_name).await?;
        Ok(Self {
            pool,
            table_name: table_name.to_string()
        })
    }

    async fn create_table(
        pool: &Pool<PostgresConnectionManager<Tls>>,
        table_name: &str,
    ) -> anyhow::Result<()> {
        let client = pool.get().await?;
        client
            .simple_query(&format!(
                "CREATE TABLE IF NOT EXISTS {} (
            aggregate_id    UUID NOT NULL,
            event_id        BIGINT NOT NULL,
            payload         JSONB,
            PRIMARY KEY (aggregate_id, event_id)
          )",
                table_name
            ))
            .await?;
        Ok(())
    }
}

impl PostgresStorage<NoTls> {
    pub async fn from_connection_string(connection_string: &str, table_name: &str) -> anyhow::Result<Self> {

        let config = tokio_postgres::config::Config::from_str(connection_string)?;
        let pg_mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);

        let pool = Pool::builder().test_on_check_out(true).min_idle(Some(4)).build(pg_mgr).await?;

        Self::create_table(&pool, table_name).await?;

        Ok(Self {
            pool,
            table_name: table_name.to_string()
        })
    }
}

#[async_trait::async_trait]
impl<P: Send + Sync + Serialize + DeserializeOwned, Tls> Storage<P> for PostgresStorage<Tls>
    where
        Tls: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <Tls as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <Tls as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<Tls as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    async fn events_for_aggregate<'a>(&self, aggregate_id: &uuid::Uuid) -> BoxStream<'a, anyhow::Result<(u64, P)>> where P: 'a {
        match self.pool.get().await {
            Ok(client) => {
                match client
                    .query_raw(format!(
                        "SELECT event_id, payload \
                             FROM {} \
                             WHERE aggregate_id = $1 \
                             ORDER BY event_id ASC;",
                        self.table_name
                    ).as_str(), &[&aggregate_id])
                    .await {
                    Ok(stream) => {
                        stream.map(|row_res| {
                            match row_res {
                                Ok(row) => {
                                    let event_id = row.get::<usize, i64>(0) as u64;
                                    let raw_payload = row.get(1);

                                    match serde_json::from_value::<P>(raw_payload) {
                                        Ok(payload) => {
                                            Ok((event_id, payload))
                                        },
                                        Err(err) => Err(err.into())
                                    }

                                }
                                Err(err) => Err(err.into())
                            }
                        }).boxed()
                    }
                    Err(err) => stream::iter(vec![Err(err.into())]).boxed()
                }
            }
            Err(err) => stream::iter(vec![Err(err.into())]).boxed()
        }
    }

    async fn total_events_for_aggregate(&self, aggregate_id: &uuid::Uuid) -> anyhow::Result<u64> {
        let client = self.pool.get().await?;
        let row_list = client.query(&format!("SELECT COUNT(*) FROM {} WHERE aggregate_id='{}'", self.table_name, aggregate_id), &[]).await?;
        Ok(row_list[0].try_get::<_, i64>(0)? as u64)
    }

    async fn total_events(&self) -> anyhow::Result<u64> {
        let client = self.pool.get().await?;
        let row_list = client.query(&format!("SELECT COUNT(*) FROM {}", self.table_name), &[]).await?;
        Ok(row_list[0].try_get::<_, i64>(0)? as u64)
    }

    async fn save_events<'a>(&self, aggregate_id: &uuid::Uuid, events: Vec<(u64, P)>) -> eventific::SaveEventsResult where P: 'a {
        match self.pool.get().await {
            Ok(mut client) => {
                match client.transaction().await {
                    Ok(transaction) => {
                        match transaction.prepare(&format!(
                            "INSERT INTO {} (aggregate_id, event_id, payload)\
                 VALUES ($1, $2, $3)", self.table_name))
                            .await {
                            Ok(statement) => {
                                for event in events {
                                    match serde_json::to_value(event.1) {
                                        Ok(payload) => {
                                            if let Err(err) = transaction.execute(&statement, &[
                                                aggregate_id,
                                                &(event.0 as i64),
                                                &payload,
                                            ])
                                                .await {

                                                if let Some(code) = err.code() {
                                                    if code.code() == "23505" {
                                                        return SaveEventsResult::VersionConflict
                                                    }
                                                }

                                                return SaveEventsResult::Error(err.into());
                                            }
                                        }
                                        Err(err) => {
                                            return SaveEventsResult::Error(err.into())
                                        }
                                    }
                                }

                                match transaction.commit()
                                    .await {
                                    Ok(_) => SaveEventsResult::Ok,
                                    Err(err) => SaveEventsResult::Error(err.into())
                                }
                            }
                            Err(err) => SaveEventsResult::Error(err.into())
                        }
                    }
                    Err(err) => SaveEventsResult::Error(err.into())
                }
            }
            Err(err) => SaveEventsResult::Error(err.into())
        }
    }

    async fn aggregate_version(&self, aggregate_id: &uuid::Uuid) -> anyhow::Result<u64> {
        let client = self.pool.get().await?;
        let row_list = client.query(&format!("SELECT event_id FROM {} WHERE aggregate_id = $1 ORDER BY event_id DESC;", self.table_name), &[aggregate_id]).await?;
        Ok(row_list[0].try_get::<_, i64>(0)? as u64)
    }

    async fn all_aggregate_ids(&self) -> futures::stream::BoxStream<'_, anyhow::Result<uuid::Uuid>> {
        match self.pool.get().await {
            Ok(client) => {
                match client
                    .query_raw::<_, _, &Vec<String>>(format!(
                        "SELECT DISTINCT aggregate_id FROM {}",
                        self.table_name
                    ).as_str(), &vec![])
                    .await {
                    Ok(stream) => {
                        stream.map_ok(|row| {
                            row.get(0)
                        }).map_err(|i| i.into()).boxed()
                    }
                    Err(err) => stream::iter(vec![Err(err.into())]).boxed()
                }
            }
            Err(err) => stream::iter(vec![Err(err.into())]).boxed()
        }
    }
}
