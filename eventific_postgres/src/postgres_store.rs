use eventific::store::{Store, StoreError};
use eventific::{Event, EventData};
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

#[derive(Clone)]
pub struct PostgresStore<D> {
    connection_string: String,
    service_name: String,
    client: Option<Arc<RwLock<Client>>>,
    phantom: PhantomData<D>,
}

impl<D> PostgresStore<D> {
    pub fn new(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_owned(),
            service_name: "".to_owned(),
            client: None,
            phantom: PhantomData,
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

impl<
        D: EventData + Serialize + DeserializeOwned,
        M: 'static + Send + Sync + Debug + Serialize + DeserializeOwned,
    > Store<D, M> for PostgresStore<D>
{
    fn init<'a>(
        &'a mut self,
        logger: &'a Logger,
        service_name: &str,
    ) -> BoxFuture<'a, Result<(), StoreError<D, M>>> {
        self.service_name = service_name.to_owned();
        async move {
            info!(logger, "Initializing postgres store");
            let (client, connection) = tokio_postgres::connect(&self.connection_string, NoTls)
                .await
                .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

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

            Self::create_table(&logger, &client, &self.service_name)
                .await
                .map_err(|err| StoreError::ConnectError(format_err!("{:?}", err)))?;

            Ok(())
        }
        .boxed()
    }

    fn save_events<'a>(
        &'a self,
        logger: &'a Logger,
        events: Vec<Event<D, M>>,
    ) -> BoxFuture<'a, Result<(), StoreError<D, M>>> {
        async move {
            if !events.is_empty() {
                info!(logger, "Persisting events");

                let mut client = self.client.as_ref().expect("Store has not been initialized").write().await;
                let service_name = self.service_name.to_owned();

                let transaction = client.transaction()
                    .await
                    .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

                let statement = transaction.prepare(&format!(
                    "INSERT INTO {}_event_store (aggregate_id, event_id, created_date, metadata, payload)\
                 VALUES ($1, $2, $3, $4, $5)", service_name))
                    .await
                    .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

                for event in events {
                    transaction.execute(&statement, &[
                        &event.aggregate_id,
                        &(event.event_id as i32),
                        &event.created_date,
                        &serde_json::to_value(&event.metadata).unwrap(),
                        &serde_json::to_value(&event.payload).unwrap()
                    ])
                        .await
                        .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;
                }

                transaction.commit()
                    .await
                    .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

                Ok(())
            } else {
                warn!(logger, "No events to persist, skipping...");
                Ok(())
            }
        }.boxed()
    }

    fn events<'a>(
        &'a self,
        logger: &'a Logger,
        aggregate_id: Uuid,
    ) -> BoxFuture<'a, Result<BoxStream<'a, Result<Event<D, M>, StoreError<D, M>>>, StoreError<D, M>>>
    {
        async move {
            info!(logger, "Starting to tail the event log");

            let client = self
                .client
                .as_ref()
                .expect("Store has not been initialized")
                .read()
                .await;
            let service_name = self.service_name.to_owned();

            let statement = client
                .prepare(&format!(
                    "SELECT event_id, created_date, metadata, payload \
                             FROM {}_event_store \
                             WHERE aggregate_id = $1 \
                             ORDER BY event_id ASC;",
                    service_name
                ))
                .await
                .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

            let params = vec![aggregate_id];
            let row_stream = client
                .query_raw(&statement, params.iter().map(|p| p as &dyn ToSql))
                .await
                .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

            let event_stream: BoxStream<_> = row_stream
                .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))
                .and_then(move |row| async move {
                    Ok(Event {
                        aggregate_id,
                        event_id: row.get::<usize, i32>(0) as u32,
                        created_date: row.get(1),
                        metadata: serde_json::from_value::<Option<M>>(row.get(2))
                            .map_err(|e| StoreError::Unknown(format_err!("{}", e)))?,
                        payload: serde_json::from_value::<D>(row.get(3))
                            .map_err(|e| StoreError::Unknown(format_err!("{}", e)))?,
                    })
                })
                .boxed();

            Ok(event_stream)
        }
        .boxed()
    }

    fn aggregate_ids<'a>(
        &'a self,
        _logger: &'a Logger,
    ) -> BoxFuture<'a, Result<BoxStream<'a, Result<Uuid, StoreError<D, M>>>, StoreError<D, M>>>
    {
        async move {
            let client = self
                .client
                .as_ref()
                .expect("Store has not been initialized")
                .read()
                .await;
            let service_name = self.service_name.to_owned();

            let statement = client
                .prepare(&format!(
                    "SELECT DISTINCT aggregate_id FROM {}_event_store",
                    service_name
                ))
                .await
                .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

            let params: Vec<String> = vec![];
            let row_stream = client
                .query_raw(&statement, params.iter().map(|p| p as &dyn ToSql))
                .await
                .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

            let stream: BoxStream<_> = row_stream
                .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))
                .map_ok(|row| row.get(0))
                .boxed();

            Ok(stream)
        }
        .boxed()
    }

    fn total_aggregates<'a>(
        &'a self,
        _logger: &'a Logger,
    ) -> BoxFuture<'a, Result<u64, StoreError<D, M>>> {
        async move {
            let client = self
                .client
                .as_ref()
                .expect("Store has not been initialized")
                .read()
                .await;
            let service_name = self.service_name.to_owned();

            let statement = client
                .prepare(&format!(
                    "SELECT COUNT(DISTINCT aggregate_id) FROM {}_event_store",
                    service_name
                ))
                .await
                .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

            let rows = client
                .query(&statement, &[])
                .await
                .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

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
        _logger: &'a Logger,
        aggregate_id: Uuid,
    ) -> BoxFuture<'a, Result<u64, StoreError<D, M>>> {
        async move {
            let client = self
                .client
                .as_ref()
                .expect("Store has not been initialized")
                .read()
                .await;
            let service_name = self.service_name.to_owned();

            let statement = client
                .prepare(&format!(
                    "SELECT COUNT(*) FROM {}_event_store WHERE aggregate_id = $1",
                    service_name
                ))
                .await
                .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

            let rows = client
                .query(&statement, &[&aggregate_id])
                .await
                .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

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
        _logger: &'a Logger,
    ) -> BoxFuture<'a, Result<u64, StoreError<D, M>>> {
        async move {
            let client = self
                .client
                .as_ref()
                .expect("Store has not been initialized")
                .read()
                .await;
            let service_name = self.service_name.to_owned();

            let statement = client
                .prepare(&format!(
                    "SELECT COUNT(*) FROM {}_event_store",
                    service_name
                ))
                .await
                .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

            let rows = client
                .query(&statement, &[])
                .await
                .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))?;

            let count = match rows.first() {
                None => 0,
                Some(row) => row.get::<usize, i64>(0) as u64,
            };

            Ok(count)
        }
        .boxed()
    }
}
