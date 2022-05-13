use uuid::Uuid;
use futures::stream::BoxStream;
use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::SaveEventsResult;


#[async_trait::async_trait]
pub trait Storage<P: Send + Sync>: Send + Sync {
    async fn events_for_aggregate<'a>(&self, aggregate_id: &Uuid) -> BoxStream<'a, anyhow::Result<(u64, P)>> where P: 'a;
    async fn total_events_for_aggregate(&self, aggregate_id: &Uuid) -> anyhow::Result<u64>;
    async fn total_events(&self) -> anyhow::Result<u64>;
    async fn save_events<'a>(&self, aggregate_id: &Uuid, events: Vec<(u64, P)>) -> SaveEventsResult where P: 'a;
    async fn aggregate_version(&self, aggregate_id: &Uuid) -> anyhow::Result<u64>;
    async fn all_aggregate_ids(&self) -> BoxStream<'_, anyhow::Result<Uuid>>;
}

#[cfg(test)]
pub mod test {
    use crate::Storage;
    use uuid::Uuid;
    use futures::prelude::stream::BoxStream;
    use alloc::vec::Vec;
    use crate::storage::SaveEventsResult;
    use alloc::boxed::Box;
    use futures::stream;
    use futures::stream::StreamExt;
    use core::marker::PhantomData;

    pub struct MockStorage<P> {
        phantom: PhantomData<P>
    }

    #[async_trait::async_trait]
    impl<P: Send + Sync> Storage<P> for MockStorage<P> {
        async fn events_for_aggregate<'a>(&self, _aggregate_id: &Uuid) -> BoxStream<'a, anyhow::Result<(u64, P)>> where P: 'a {
            stream::iter(Vec::new()).boxed()
        }

        async fn total_events_for_aggregate(&self, _aggregate_id: &Uuid) -> anyhow::Result<u64> {
            Ok(0)
        }

        async fn total_events(&self) -> anyhow::Result<u64> {
            Ok(0)
        }

        async fn save_events<'a>(&self, _aggregate_id: &Uuid, _events: Vec<(u64, P)>) -> SaveEventsResult where P: 'a {
            SaveEventsResult::Ok
        }

        async fn aggregate_version(&self, _aggregate_id: &Uuid) -> anyhow::Result<u64> {
            Ok(0)
        }

        async fn all_aggregate_ids(&self) -> BoxStream<'_, anyhow::Result<Uuid>> {
            stream::iter(Vec::new()).boxed()
        }
    }

    impl<P> Default for MockStorage<P> {
        fn default() -> Self {
            Self {
                phantom: PhantomData
            }
        }
    }


}
