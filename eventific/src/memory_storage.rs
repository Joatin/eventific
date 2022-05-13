use crate::storage::{Storage};
use uuid::Uuid;
use futures::prelude::stream::BoxStream;
use futures::stream::iter;
use futures::StreamExt;
use tokio::sync::RwLock;
use alloc::collections::BTreeMap;
use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::SaveEventsResult;

pub struct MemoryStorage<P> {
    aggregates: RwLock<BTreeMap<Uuid, BTreeMap<u64, P>>>
}

impl<P> MemoryStorage<P> {
    pub fn new() -> Self {
        let aggregates = RwLock::new(BTreeMap::new());
        Self {
            aggregates
        }
    }
}

#[async_trait::async_trait]
impl<P: Send + Sync + Clone> Storage<P> for MemoryStorage<P> {
    async fn events_for_aggregate<'a>(&self, aggregate_id: &Uuid) -> BoxStream<'a, anyhow::Result<(u64, P)>> where P: 'a {
        let lock = self.aggregates.read().await;
        match lock.get(aggregate_id) {
            Some(events) => {
                iter(events.clone().into_iter()).map(Ok).boxed()
            },
            None => {
                iter(Vec::new()).map(Ok).boxed()
            }
        }
    }

    async fn total_events_for_aggregate(&self, aggregate_id: &Uuid) -> anyhow::Result<u64> {
        let lock = self.aggregates.read().await;
        match lock.get(aggregate_id) {
            None => {
                Ok(0)
            }
            Some(events) => {
                Ok(events.len() as u64)
            }
        }
    }

    async fn total_events(&self) -> anyhow::Result<u64> {
        let lock = self.aggregates.read().await;

        let total_events = lock.values().fold(0, |acc, events| {
            events.len() + acc
        });

        Ok(total_events as u64)
    }

    async fn save_events<'a>(&self, aggregate_id: &Uuid, events: Vec<(u64, P)>) -> SaveEventsResult where P: 'a {
        let mut lock = self.aggregates.write().await;

        match lock.get_mut(aggregate_id) {
            None => {
                let events_map = events.into_iter().collect();
                lock.insert(*aggregate_id, events_map);
                SaveEventsResult::Ok
            }
            Some(events_map) => {
                for (key, _) in &events {
                    if events_map.contains_key(key) {
                        return SaveEventsResult::VersionConflict;
                    }
                }
                for (key, val) in events {
                    events_map.insert(key, val);
                }
                SaveEventsResult::Ok
            }
        }
    }

    async fn aggregate_version(&self, aggregate_id: &Uuid) -> anyhow::Result<u64> {
        let lock = self.aggregates.read().await;
        match lock.get(aggregate_id) {
            None => {
                Ok(0)
            }
            Some(events) => {
                Ok(events.len() as u64)
            }
        }
    }

    async fn all_aggregate_ids(&self) -> BoxStream<'_, anyhow::Result<Uuid>> {
        let keys: Vec<Uuid> = {
            let lock = self.aggregates.read().await;
            lock.keys().cloned().collect()
        };
        iter(keys).map(Ok).boxed()
    }
}
