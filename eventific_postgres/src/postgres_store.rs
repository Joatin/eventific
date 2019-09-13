use eventific::store::{Store, StoreError};
use slog::Logger;
use futures::{Future, Stream};
use eventific::event::Event;
use uuid::Uuid;
use std::marker::PhantomData;
use std::fmt::Debug;
use tokio_postgres::{NoTls, Client};
use std::sync::{Arc, RwLock};
use futures::future::{loop_fn, Either};
use bb8::{Builder, Pool};
use bb8_postgres::PostgresConnectionManager;
use tokio::timer::Delay;
use std::time::{Instant, Duration};
use futures::future::Loop;
use futures::future::lazy;
use futures::future::join_all;
use std::ops::Add;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use futures::sync::mpsc::channel;
use futures::sink::Sink;

#[derive(Clone)]
pub struct PostgresStore<D> {
    config: String,
    service_name: String,
    logger: Option<Logger>,
    pool: Arc<RwLock<Option<Pool<PostgresConnectionManager<NoTls>>>>>,
    phantom: PhantomData<D>
}

impl<D> PostgresStore<D> {
    const MAX_RETRIES: u64 = 7;

    pub fn new(config: &str) -> Self {
        Self {
            config: config.to_owned(),
            service_name: "".to_owned(),
            logger: None,
            pool: Arc::new(RwLock::new(None)),
            phantom: PhantomData
        }
    }

    fn create_table(logger: &Logger, mut client: Client, service_name: &str) -> impl Future<Item = ((), Client), Error = (tokio_postgres::Error, Client)> {
        let log = logger.clone();
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
            .into_future()
            .then(move |res| {
                match res {
                    Ok(_) => {
                        info!(log, "Created new table to hold the events");
                        Ok(((), client))
                    },
                    Err((err, _)) => {
                        Err((err, client))
                    },
                }
            })
    }
}

impl<D: 'static + Send + Sync + Debug + Clone + Serialize + DeserializeOwned> Store<D> for PostgresStore<D> {
    fn init(&mut self, logger: &Logger, service_name: &str) -> Box<dyn Future<Item=(), Error=StoreError<D>> + Send> {
        self.service_name = service_name.to_owned();
        self.logger.replace(logger.clone());
        let s_name = service_name.to_owned();
        let pool_opt = Arc::clone(&self.pool);
        let pg_mgr = PostgresConnectionManager::new(&self.config, NoTls);
        let logz = logger.clone();
        let logz2 = logger.clone();
        let res_fut = lazy(move|| {
            info!(logz, "Setting up new Postgres Store");
            loop_fn((pg_mgr, 0, logz.clone()), |(manager, r_count, log)| {
                Builder::new()
                    .min_idle(Some(4))
                    .max_lifetime(Some(Duration::from_secs(300)))
                    .connection_timeout(Duration::from_millis(500))
                    .max_size(30)
                    .build(manager.clone())
                    .map_err(|err| StoreError::ConnectError(format_err!("{}", err)))
                    .then(move |pool_res| {
                        Delay::new(Instant::now().add(Duration::from_millis((r_count + 1) * (3_000 / Self::MAX_RETRIES))))
                            .map_err(|err| StoreError::Unknown(format_err!("{}", err)))
                            .and_then(move |_| {
                                match pool_res {
                                    Ok(pool) => {
                                        info!(log, "Postgres connection pool has successfully been constructed");
                                        Ok(Loop::Break(pool))
                                    },
                                    Err(err) => {
                                        if r_count < Self::MAX_RETRIES {
                                            warn!(log, "Failed to connect to database, retrying"; "error" => format!("{}", err));
                                            Ok(Loop::Continue((manager, r_count + 1, log)))
                                        } else {
                                            error!(log, "Failed to connect to database, giving up!"; "error" => format!("{}", err));
                                            Err(err)
                                        }
                                    }
                                }
                            })
                    })
            })
                .and_then(move |pool| {
                    pool.run(move |client| {
                        Self::create_table(&logz2, client, &s_name)
                    })
                        .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)))
                        .and_then(move |_| {
                            {
                                let mut lock = pool_opt.write().unwrap();
                                lock.replace(pool);
                            }
                            Ok(())
                        })

                })
        });

        Box::new(res_fut)
    }

    fn save_events(&self, events: Vec<Event<D>>) -> Box<dyn Future<Item=(), Error=StoreError<D>> + Send> {
        let lock = self.pool.read().unwrap();
        let pool = lock.as_ref().expect("Store has not been initialized");
        let service_name = self.service_name.to_owned();

        let res_fut = pool.run(move |mut client| {
            client.prepare(&format!(
                "INSERT INTO {}_event_store (aggregate_id, event_id, created_date, metadata, payload)\
      VALUES ($1, $2, $3, $4, $5)", service_name))
                .then(|statement_res| {
                    match statement_res {
                        Ok(statement) => {
                            let mut query_futures = Vec::with_capacity(events.len());

                            for event in events {
                                query_futures.push(client.execute(&statement, &[
                                    &event.aggregate_id,
                                    &(event.event_id as i32),
                                    &event.created_date,
                                    &serde_json::to_value(&event.metadata).unwrap(),
                                    &serde_json::to_value(&event.payload).unwrap()
                                ]))
                            }

                            let queries = join_all(query_futures);

                            let trans_fut = client.build_transaction()
                                .build(queries)
                                .then(move |res| {
                                    match res {
                                        Ok(_) => {
                                            Ok(((), client))
                                        },
                                        Err(err) => {
                                            Err((err, client))
                                        },
                                    }
                                });
                            Either::A(trans_fut)
                        },
                        Err(err) => {
                            Either::B(futures::failed((err, client)))
                        },
                    }

            })

        })
            .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)));

        Box::new(res_fut)
    }

    fn events(&self, aggregate_id: Uuid) -> Box<dyn Future<Item=Vec<Event<D>>, Error=StoreError<D>> + Send> {
        let lock = self.pool.read().unwrap();
        let pool = lock.as_ref().expect("Store has not been initialized");
        let service_name = self.service_name.to_owned();

        let res_fut = pool.run(move |mut client| {
            client.prepare(&format!(
                "SELECT event_id, created_date, metadata, payload \
                             FROM {}_event_store \
                             WHERE aggregate_id = $1 \
                             ORDER BY event_id ASC;", service_name))
                .then(move |statement_res| {
                    match statement_res {
                        Ok(statement) => {

                            let q_fut = client.query(&statement, &[
                                &aggregate_id
                            ])
                                .map_err(|err| format_err!("{}", err))
                                .and_then(move |row| {
                                    Ok(Event {
                                        aggregate_id,
                                        event_id: row.get::<usize, i32>(0) as u32,
                                        created_date: row.get(1),
                                        metadata: serde_json::from_value::<HashMap<String, String>>(
                                            row.get(2),
                                        ).map_err(|e| format_err!("{}", e))?,
                                        payload: serde_json::from_value::<D>(
                                            row.get(3),
                                        ).map_err(|e| format_err!("{}", e))?
                                    })
                                })
                                .collect()
                                .then(move |res| {
                                    match res {
                                        Ok(evs) => {
                                            Ok((evs, client))
                                        },
                                        Err(err) => {
                                            Err((err, client))
                                        },
                                    }
                                });

                            Either::A(q_fut)
                        },
                        Err(err) => {
                            Either::B(futures::failed((format_err!("{}", err), client)))
                        },
                    }

                })
        })
            .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)));

        Box::new(res_fut)
    }

    fn aggregate_ids(&self) -> Box<dyn Stream<Item=Uuid, Error=StoreError<D>> + Send> {
        let lock = self.pool.read().unwrap();
        let pool = lock.as_ref().expect("Store has not been initialized");
        let service_name = self.service_name.to_owned();
        let logger = self.logger.clone().expect("Store has not been initialized");
        let err_log = logger.clone();

        let (sx, rx) = channel::<Uuid>(10000);

        let res_fut = pool.run(move |mut client| {
            client.prepare(&format!(
                "SELECT DISTINCT aggregate_id FROM {}_event_store", service_name))
                .then(move |statement_res| {
                    match statement_res {
                        Ok(statement) => {
                            let q_fut = client.query(&statement, &[])
                                .map_err(|err| format_err!("{}", err))
                                .and_then(move |row| {
                                    let id: Uuid = row.get(0);
                                    sx.clone()
                                        .send(id)
                                        .map(|_|())
                                        .map_err(|err| format_err!("{}", err))
                                })
                                .collect()
                                .then(move |res| {
                                    match res {
                                        Ok(_) => {
                                            Ok(((), client))
                                        },
                                        Err(err) => {
                                            Err((err, client))
                                        },
                                    }
                                });

                            Either::A(q_fut)
                        },
                        Err(err) => {
                            Either::B(futures::failed((format_err!("{}", err), client)))
                        },
                    }

                })
        })
            .map_err(move |err| error!(err_log, "{:?}", err));

        tokio::spawn(res_fut);

        Box::new(rx.map_err(|_| StoreError::Unknown(format_err!("Channel closed"))))
    }

    fn total_aggregates(&self) -> Box<dyn Future<Item=u64, Error=StoreError<D>> + Send> {
        let lock = self.pool.read().unwrap();
        let pool = lock.as_ref().expect("Store has not been initialized");
        let service_name = self.service_name.to_owned();
        let res_fut = pool.run(move |mut client| {
            client.prepare(&format!(
                "SELECT COUNT(DISTINCT aggregate_id) FROM {}_event_store", service_name))
                .then(move |statement_res| {
                    match statement_res {
                        Ok(statement) => {
                            let q_fut = client.query(&statement, &[])
                                .map_err(|err| format_err!("{}", err))
                                .and_then(move |row| {
                                    Ok(row.get::<usize, i64>(0) as u64)
                                })
                                .collect()
                                .then(move |res| {
                                    match res {
                                        Ok(count) => {
                                            Ok((count[0], client))
                                        },
                                        Err(err) => {
                                            Err((err, client))
                                        },
                                    }
                                });

                            Either::A(q_fut)
                        },
                        Err(err) => {
                            Either::B(futures::failed((format_err!("{}", err), client)))
                        },
                    }

                })
        })
            .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)));

        Box::new(res_fut)
    }

    fn total_events_for_aggregate(&self, aggregate_id: Uuid) -> Box<dyn Future<Item=u64, Error=StoreError<D>> + Send> {
        let lock = self.pool.read().unwrap();
        let pool = lock.as_ref().expect("Store has not been initialized");
        let service_name = self.service_name.to_owned();
        let res_fut = pool.run(move |mut client| {
            client.prepare(&format!(
                "SELECT COUNT(*) FROM {}_event_store WHERE aggregate_id = $1", service_name))
                .then(move |statement_res| {
                    match statement_res {
                        Ok(statement) => {
                            let q_fut = client.query(&statement, &[
                                &aggregate_id
                            ])
                                .map_err(|err| format_err!("{}", err))
                                .and_then(move |row| {
                                    Ok(row.get::<usize, i64>(0) as u64)
                                })
                                .collect()
                                .then(move |res| {
                                    match res {
                                        Ok(count) => {
                                            Ok((count[0], client))
                                        },
                                        Err(err) => {
                                            Err((err, client))
                                        },
                                    }
                                });

                            Either::A(q_fut)
                        },
                        Err(err) => {
                            Either::B(futures::failed((format_err!("{}", err), client)))
                        },
                    }

                })
        })
            .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)));

        Box::new(res_fut)
    }

    fn total_events(&self) -> Box<dyn Future<Item=u64, Error=StoreError<D>> + Send> {
        let lock = self.pool.read().unwrap();
        let pool = lock.as_ref().expect("Store has not been initialized");
        let service_name = self.service_name.to_owned();
        let res_fut = pool.run(move |mut client| {
            client.prepare(&format!(
                "SELECT COUNT(*) FROM {}_event_store", service_name))
                .then(move |statement_res| {
                    match statement_res {
                        Ok(statement) => {
                            let q_fut = client.query(&statement, &[])
                                .map_err(|err| format_err!("{}", err))
                                .and_then(move |row| {
                                    Ok(row.get::<usize, i64>(0) as u64)
                                })
                                .collect()
                                .then(move |res| {
                                    match res {
                                        Ok(count) => {
                                            Ok((count[0], client))
                                        },
                                        Err(err) => {
                                            Err((err, client))
                                        },
                                    }
                                });

                            Either::A(q_fut)
                        },
                        Err(err) => {
                            Either::B(futures::failed((format_err!("{}", err), client)))
                        },
                    }

                })
        })
            .map_err(|err| StoreError::Unknown(format_err!("{:?}", err)));

        Box::new(res_fut)
    }
}
