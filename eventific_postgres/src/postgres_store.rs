use eventific::store::{Store, StoreContext, SaveEventsResult};
use eventific::{Event};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::stream::StreamExt;
use futures::{FutureExt, TryStreamExt};
use serde::de::DeserializeOwned;
use serde::Serialize;
use slog::Logger;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, NoTls};
use uuid::Uuid;
use std::error::Error;
use crate::postgres_store_error::PostgresStoreError;

#[derive(Clone)]
pub struct PostgresStore<D, M> {
    connection_string: String,
    client: Option<Arc<RwLock<Client>>>,
    phantomEventData: PhantomData<D>,
    phantomMetaData: PhantomData<M>,
}

impl<D, M> PostgresStore<D, M> {
    pub fn new(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_owned(),
            client: None,
            phantomEventData: PhantomData,
            phantomMetaData: PhantomData,
        }
    }

    async fn create_table(
        logger: &Logger,
        client: &Client,
        service_name: &str,
    ) -> Result<(), tokio_postgres::Error> {
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
            .await?;
        info!(logger, "Created new table to hold the events");
        Ok(())
    }
}

impl<D: 'static + Send + Sync + DeserializeOwned + Serialize, M: 'static + Send + Sync + DeserializeOwned + Serialize> Store for PostgresStore<D, M>
{
    type Error = PostgresStoreError;
    type EventData = D;
    type MetaData = M;

    fn init<'a>(
        &'a mut self,
        context: StoreContext
    ) -> BoxFuture<'a, Result<(), Self::Error>> {
        async move {
            info!(context.logger, "Initializing postgres store");
            let (client, connection) = tokio_postgres::connect(&self.connection_string, NoTls)
                .await
                .map_err(PostgresStoreError::ClientError)?;

            self.client.replace(Arc::new(RwLock::new(client)));

            let client = self
                .client
                .as_ref()
                .expect("Store has not been initialized")
                .read()
                .await;

            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                    panic!()
                }
            });

            Self::create_table(&context.logger, &client, &context.service_name)
                .await
                .map_err(PostgresStoreError::CreateTableError)?;

            Ok(())
        }
        .boxed()
    }

    fn save_events<'a>(
        &'a self,
        context: StoreContext,
        events: &'a Vec<Event<Self::EventData, Self::MetaData>>,
    ) -> BoxFuture<'a, Result<SaveEventsResult, Self::Error>> {
        async move {
            if !events.is_empty() {
                info!(context.logger, "Persisting events");

                let mut client = self.client.as_ref().expect("Store has not been initialized").write().await;
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
                warn!(context.logger, "No events to persist, skipping...");
                Ok(SaveEventsResult::AlreadyExists)
            }
        }.boxed()
    }

    fn events<'a>(
        &'a self,
        context: StoreContext,
        aggregate_id: Uuid,
    ) -> BoxFuture<'a, Result<BoxStream<'a, Result<Event<D, M>, Self::Error>>, Self::Error>>
    {
        async move {
            info!(context.logger, "Starting to tail the event log");

            let client = self
                .client
                .as_ref()
                .expect("Store has not been initialized")
                .read()
                .await;
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
        .boxed()
    }

    fn aggregate_ids<'a>(
        &'a self,
        context: StoreContext
    ) -> BoxFuture<'a, Result<BoxStream<'a, Result<Uuid, Self::Error>>, Self::Error>>
    {
        async move {
            let client = self
                .client
                .as_ref()
                .expect("Store has not been initialized")
                .read()
                .await;
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
        .boxed()
    }

    fn total_aggregates<'a>(
        &'a self,
        context: StoreContext,
    ) -> BoxFuture<'a, Result<u64, Self::Error>> {
        async move {
            let client = self
                .client
                .as_ref()
                .expect("Store has not been initialized")
                .read()
                .await;
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
        .boxed()
    }

    fn total_events_for_aggregate<'a>(
        &'a self,
        context: StoreContext,
        aggregate_id: Uuid,
    ) -> BoxFuture<'a, Result<u64, Self::Error>> {
        async move {
            let client = self
                .client
                .as_ref()
                .expect("Store has not been initialized")
                .read()
                .await;
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
        .boxed()
    }

    fn total_events<'a>(
        &'a self,
        context: StoreContext,
    ) -> BoxFuture<'a, Result<u64, Self::Error>> {
        async move {
            let client = self
                .client
                .as_ref()
                .expect("Store has not been initialized")
                .read()
                .await;
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
        .boxed()
    }
}
