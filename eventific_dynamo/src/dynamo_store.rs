use eventific::store::{Store, StoreError};
use futures::{Future, Stream};
use eventific::event::Event;
use uuid::Uuid;
use slog::Logger;
use rusoto_core::RusotoError;
use rusoto_dynamodb::{DynamoDbClient, DynamoDb, TransactWriteItemsInput, TransactWriteItem, Put, CreateTableInput, CreateTableError, TransactWriteItemsError, AttributeValue, QueryInput};
use std::fmt::Debug;
use std::sync::Arc;
use std::str::FromStr;
use crate::Region;
use std::marker::PhantomData;
use std::collections::HashMap;
use chrono::Utc;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Clone)]
pub struct DynamoStore<D: Clone> {
    region: Region,
    client: Option<Arc<DynamoDbClient>>,
    logger: Option<Logger>,
    table_name: String,
    service_name: Option<String>,
    phantom: PhantomData<D>
}

impl<D: 'static + Send + Sync + Debug + Clone> DynamoStore<D> {
    pub fn new(region: &str, table_name: Option<String>) -> Self {
        let parsed_region = Region::from_str(region).expect("Not a valid region");
        Self {
            region: parsed_region,
            client: None,
            logger: None,
            table_name: table_name.unwrap_or_else(|| std::env::var("TABLE_NAME").expect("If no table name is provide then the TABLE_NAME env variable must be set!")),
            service_name: None,
            phantom: PhantomData
        }
    }

    pub fn new_from_region(region: Region, table_name: Option<String>) -> Self {
        Self {
            region,
            client: None,
            logger: None,
            table_name: table_name.unwrap_or_else(|| std::env::var("TABLE_NAME").expect("If no table name is provide then the TABLE_NAME env variable must be set!")),
            service_name: None,
            phantom: PhantomData
        }
    }

    fn table_name(&self) -> String {
        format!("{}", self.table_name)
    }

    fn map_rusoto_err<T: 'static + std::error::Error>(err: RusotoError<T>) -> StoreError<D> {
        match err {
            RusotoError::Service(_) => {
                StoreError::Unknown(format_err!("{}", err))
            },
            RusotoError::HttpDispatch(e) => {
                StoreError::ConnectError(format_err!("{}", e))
            },
            RusotoError::Credentials(e) => {
                StoreError::CredentialsError(format_err!("{}", e))
            },
            _ => {
                StoreError::Unknown(format_err!("{}", err))
            }
        }
    }
}

impl<D: 'static + Send + Sync + Debug + Clone + DeserializeOwned + Serialize> Store<D> for DynamoStore<D> {
    fn init(&mut self, logger: &Logger, service_name: &str) -> Box<Future<Item=(), Error=StoreError<D>> + Send> {
        let client = DynamoDbClient::new(self.region.clone());
        self.logger.replace(logger.clone());
        self.client.replace(Arc::new(client));
        self.service_name.replace(service_name.to_owned());

        Box::new(futures::finished(()))
    }

    fn save_events(&self, events: Vec<Event<D>>) -> Box<Future<Item=(), Error=StoreError<D>> + Send> {
        let logger = self.logger.clone().expect("The store has to be initialized");
        let client = self.client.as_ref().expect("The store has to be initialized");

        if events.len() > 10 {
            return Box::new(futures::failed(StoreError::Unsupported("This store can not save more than 10 events at a time".to_owned())))
        }

        if !events.is_empty() {
            let input = TransactWriteItemsInput {
                client_request_token: None,
                return_consumed_capacity: None,
                return_item_collection_metrics: None,
                transact_items: events.into_iter().map(|event| {
                    let mut item = HashMap::new();
                    item.insert("aggregateId".to_owned(), AttributeValue {
                        b: None,
                        bool: None,
                        bs: None,
                        l: None,
                        m: None,
                        n: None,
                        ns: None,
                        null: None,
                        s: Some(event.aggregate_id.to_string()),
                        ss: None
                    });
                    item.insert("eventId".to_owned(), AttributeValue {
                        b: None,
                        bool: None,
                        bs: None,
                        l: None,
                        m: None,
                        n: Some(event.event_id.to_string()),
                        ns: None,
                        null: None,
                        s: None,
                        ss: None
                    });

                    item.insert("payload".to_owned(), AttributeValue {
                        b: None,
                        bool: None,
                        bs: None,
                        l: None,
                        m: None,
                        n: None,
                        ns: None,
                        null: None,
                        s: Some(serde_json::to_string(&event.payload).unwrap()),
                        ss: None
                    });

                    TransactWriteItem {
                        condition_check: None,
                        delete: None,
                        put: Some(Put {
                            condition_expression: None,
                            expression_attribute_names: None,
                            expression_attribute_values: None,
                            item,
                            return_values_on_condition_check_failure: None,
                            table_name: self.table_name()
                        }),
                        update: None
                    }
                }).collect()
            };

            let res_fut = client.transact_write_items(input)
                .map_err(Self::map_rusoto_err)
                .map(|_| ());

            Box::new(res_fut)
        } else {
            Box::new(futures::finished(()))
        }
    }

    fn events(&self, aggregate_id: Uuid) -> Box<Future<Item=Vec<Event<D>>, Error=StoreError<D>> + Send> {
        let logger = self.logger.clone().expect("The store has to be initialized");
        let client = self.client.as_ref().expect("The store has to be initialized");

        let mut e_values = HashMap::new();
        e_values.insert(":id".to_owned(), AttributeValue {
            b: None,
            bool: None,
            bs: None,
            l: None,
            m: None,
            n: None,
            ns: None,
            null: None,
            s: Some(aggregate_id.to_string()),
            ss: None
        });

        let res_fut = client.query(QueryInput {
            attributes_to_get: None,
            conditional_operator: None,
            consistent_read: None,
            exclusive_start_key: None,
            expression_attribute_names: None,
            expression_attribute_values: Some(e_values),
            filter_expression: None,
            index_name: None,
            key_condition_expression: Some("aggregateId = :id".to_owned()),
            key_conditions: None,
            limit: None,
            projection_expression: None,
            query_filter: None,
            return_consumed_capacity: None,
            scan_index_forward: None,
            select: None,
            table_name: self.table_name()
        })
            .then(|res| {
                match res {
                    Ok(output) => {
                        if let Some(data) = output.items {
                            let mut result = Vec::with_capacity(data.len());

                            for map in data {
                                let id = Uuid::parse_str(&map.get("aggregateId").cloned().unwrap().s.unwrap())
                                    .map_err(|e| StoreError::Unknown(format_err!("{}", e)))?;
                                result.push(Event {
                                    aggregate_id: id,
                                    event_id: u32::from_str(&map.get("eventId").cloned().unwrap().n.unwrap()).unwrap(),
                                    created_date: Utc::now(),
                                    metadata: HashMap::new(),
                                    payload: serde_json::from_str(&map.get("payload").cloned().unwrap().s.unwrap()).unwrap()
                                });
                            }
                            Ok(result)
                        } else {
                            Err(StoreError::Unknown(format_err!("No data was returned from DynamoDB")))
                        }
                    },
                    Err(err) => Err(Self::map_rusoto_err(err)),
                }
            });
        Box::new(res_fut)
    }

    fn aggregate_ids(&self) -> Box<Stream<Item=Uuid, Error=StoreError<D>> + Send> {
        Box::new(futures::empty().into_stream())
    }

    fn total_aggregates(&self) -> Box<Future<Item=u64, Error=StoreError<D>> + Send> {
        Box::new(futures::empty())
    }

    fn total_events_for_aggregate(&self, aggregate_id: Uuid) -> Box<Future<Item=u64, Error=StoreError<D>> + Send> {
        Box::new(futures::empty())
    }

    fn total_events(&self) -> Box<Future<Item=u64, Error=StoreError<D>> + Send> {
        Box::new(futures::empty())
    }
}
