use eventific::store::{Store, StoreContext, SaveEventsResult};
use eventific::{Event};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::stream::StreamExt;
use futures::{FutureExt, TryStreamExt};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, NoTls, Statement};
use eventific::Uuid;
use std::error::Error;
use crate::postgres_store_error::PostgresStoreError;
use std::str::FromStr;
use bb8_postgres::PostgresConnectionManager;
use bb8::{Pool, PooledConnection};

#[derive(Clone, Debug)]
pub struct PostgresStore<D: Debug, M: Debug> {
    connection_string: String,
    pool: Option<Pool<PostgresConnectionManager<NoTls>>>,
    phantom_event_data: PhantomData<D>,
    phantom_meta_data: PhantomData<M>,
}

impl<D: Debug, M: Debug> PostgresStore<D, M> {
    #[tracing::instrument]
    pub fn new(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_owned(),
            pool: None,
            phantom_event_data: PhantomData,
            phantom_meta_data: PhantomData
        }
    }

    #[tracing::instrument]
    async fn create_table(
        pool: &Pool<PostgresConnectionManager<NoTls>>,
        service_name: &str,
    ) -> Result<(), PostgresStoreError> {
        let client = pool.get().await
            .map_err(PostgresStoreError::PoolError)?;
        client
            .simple_query(&format!(
                "CREATE TABLE IF NOT EXISTS {}_event_store (
            aggregate_id    UUID NOT NULL,
            event_id        INT NOT NULL,
            created_date    TIMESTAMPTZ NOT NULL,
            metadata        JSONB,
            payload         JSONB,
            PRIMARY KEY (aggregate_id, event_id)
          )",
                service_name
            ))
            .await.map_err(PostgresStoreError::ClientError)?;
        info!("Created new table to hold the events");
        Ok(())
    }

    async fn get_connection(&self) -> Result<PooledConnection<'_, PostgresConnectionManager<NoTls>>, PostgresStoreError> {
        self
            .pool
            .as_ref()
            .expect("Store has not been initialized")
            .get()
            .await
            .map_err(PostgresStoreError::PoolError)
    }
}

#[eventific::async_trait]
impl<D: 'static + Send + Sync + DeserializeOwned + Serialize + Debug, M: 'static + Send + Sync + DeserializeOwned + Serialize + Debug> Store for PostgresStore<D, M>
{
    type Error = PostgresStoreError;
    type EventData = D;
    type MetaData = M;

    #[tracing::instrument]
    async fn init(
        &mut self,
        context: StoreContext
    ) -> Result<(), Self::Error> {
        info!("Initializing postgres store");
        let config = tokio_postgres::config::Config::from_str(&self.connection_string).map_err(PostgresStoreError::ClientError)?;
        let pg_mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);

        let pool = match Pool::builder().min_idle(Some(8)).build(pg_mgr).await {
            Ok(pool) => pool,
            Err(e) => panic!("builder error: {:?}", e),
        };

        Self::create_table(&pool, &context.service_name)
            .await?;

        self.pool.replace(pool);

        let mut client = self.get_connection().await?;
        let service_name = context.service_name.to_owned();
        let get_events_statement = client
            .prepare(&format!(
                "SELECT event_id, created_date, metadata, payload \
                             FROM {}_event_store \
                             WHERE aggregate_id = $1 \
                             ORDER BY event_id ASC;",
                service_name
            ))
            .await
            .map_err(PostgresStoreError::ClientError)?;

        Ok(())
    }

    #[tracing::instrument]
    async fn save_events(
        &self,
        context: StoreContext,
        events: &Vec<Event<Self::EventData, Self::MetaData>>,
    ) -> Result<SaveEventsResult, Self::Error> {
        if !events.is_empty() {
            info!("Persisting events");

            let mut client = self.get_connection().await?;
            let service_name = context.service_name.to_owned();

            let transaction = client.transaction()
                .await
                .map_err(PostgresStoreError::ClientError)?;

            let statement = transaction.prepare(&format!(
                "INSERT INTO {}_event_store (aggregate_id, event_id, created_date, metadata, payload)\
                 VALUES ($1, $2, $3, $4, $5)", service_name))
                .await
                .map_err(PostgresStoreError::ClientError)?;

            for event in events {
                transaction.execute(&statement, &[
                    &event.aggregate_id,
                    &(event.event_id as i32),
                    &event.created_date,
                    &serde_json::to_value(&event.metadata).unwrap(),
                    &serde_json::to_value(&event.payload).unwrap()
                ])
                    .await
                    .map_err(PostgresStoreError::ClientError)?;
            }

            transaction.commit()
                .await
                .map_err(PostgresStoreError::ClientError)?;

            Ok(SaveEventsResult::Success)
        } else {
            warn!("No events to persist, skipping...");
            Ok(SaveEventsResult::AlreadyExists)
        }
    }

    #[tracing::instrument]
    async fn events(
        &self,
        context: StoreContext,
        aggregate_id: Uuid,
    ) -> Result<BoxStream<'_, Result<Event<D, M>, Self::Error>>, Self::Error>
    {
        info!("Starting to tail the event log");

        let client = self.get_connection().await?;
        let service_name = context.service_name.to_owned();

        let statement = client
            .prepare(&format!(
                "SELECT event_id, created_date, metadata, payload \
                             FROM {}_event_store \
                             WHERE aggregate_id = $1 \
                             ORDER BY event_id ASC;",
                service_name
            ))
            .await
            .map_err(PostgresStoreError::ClientError)?;

        let params = vec![aggregate_id];
        let row_stream = client
            .query_raw(&statement, params.iter().map(|p| p as &dyn ToSql))
            .await
            .map_err(PostgresStoreError::ClientError)?;

        let event_stream: BoxStream<_> = row_stream
            .map_err(PostgresStoreError::ClientError)
            .and_then(move |row| async move {
                Ok(Event {
                    aggregate_id,
                    event_id: row.get::<usize, i32>(0) as u32,
                    created_date: row.get(1),
                    metadata: serde_json::from_value::<Option<M>>(row.get(2)).map_err(PostgresStoreError::SerializationError)?,
                    payload: serde_json::from_value::<D>(row.get(3)).map_err(PostgresStoreError::SerializationError)?,
                })
            })
            .boxed();

        Ok(event_stream)
    }

    #[tracing::instrument]
    async fn aggregate_ids(
        &self,
        context: StoreContext
    ) -> Result<BoxStream<'_, Result<Uuid, Self::Error>>, Self::Error>
    {
        let client = self.get_connection().await?;
        let service_name = context.service_name.to_owned();

        let statement = client
            .prepare(&format!(
                "SELECT DISTINCT aggregate_id FROM {}_event_store",
                service_name
            ))
            .await
            .map_err(PostgresStoreError::ClientError)?;

        let params: Vec<String> = vec![];
        let row_stream = client
            .query_raw(&statement, params.iter().map(|p| p as &dyn ToSql))
            .await
            .map_err(PostgresStoreError::ClientError)?;

        let stream: BoxStream<_> = row_stream
            .map_err(PostgresStoreError::ClientError)
            .map_ok(|row| row.get(0))
            .boxed();

        Ok(stream)
    }

    #[tracing::instrument]
    async fn total_aggregates(
        &self,
        context: StoreContext,
    ) -> Result<u64, Self::Error> {
        let client = self.get_connection().await?;
        let service_name = context.service_name.to_owned();

        let statement = client
            .prepare(&format!(
                "SELECT COUNT(DISTINCT aggregate_id) FROM {}_event_store",
                service_name
            ))
            .await
            .map_err(PostgresStoreError::ClientError)?;

        let rows = client
            .query(&statement, &[])
            .await
            .map_err(PostgresStoreError::ClientError)?;

        let count = match rows.first() {
            None => 0,
            Some(row) => row.get::<usize, i64>(0) as u64,
        };

        Ok(count)
    }

    #[tracing::instrument]
    async fn total_events_for_aggregate(
        &self,
        context: StoreContext,
        aggregate_id: Uuid,
    ) -> Result<u64, Self::Error> {
        let client = self.get_connection().await?;
        let service_name = context.service_name.to_owned();

        let statement = client
            .prepare(&format!(
                "SELECT COUNT(*) FROM {}_event_store WHERE aggregate_id = $1",
                service_name
            ))
            .await
            .map_err(PostgresStoreError::ClientError)?;

        let rows = client
            .query(&statement, &[&aggregate_id])
            .await
            .map_err(PostgresStoreError::ClientError)?;

        let count = match rows.first() {
            None => 0,
            Some(row) => row.get::<usize, i64>(0) as u64,
        };

        Ok(count)
    }

    #[tracing::instrument]
    async fn total_events(
        &self,
        context: StoreContext,
    ) -> Result<u64, Self::Error> {
        let client = self.get_connection().await?;
        let service_name = context.service_name.to_owned();

        let statement = client
            .prepare(&format!(
                "SELECT COUNT(*) FROM {}_event_store",
                service_name
            ))
            .await
            .map_err(PostgresStoreError::ClientError)?;

        let rows = client
            .query(&statement, &[])
            .await
            .map_err(PostgresStoreError::ClientError)?;

        let count = match rows.first() {
            None => 0,
            Some(row) => row.get::<usize, i64>(0) as u64,
        };

        Ok(count)
    }
}
