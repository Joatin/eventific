use crate::event::Event;
use crate::store::store_context::StoreContext;
use crate::store::SaveEventsResult;
use futures::future::BoxFuture;
use futures::stream::{BoxStream, TryStreamExt};
use futures::{FutureExt, StreamExt};
use std::error::Error;
use std::fmt::Debug;
use uuid::Uuid;

/// This trait is used for all component that wish to act as an event store
#[async_trait::async_trait]
pub trait Store: 'static + Send + Sync + Debug {
    type Error: 'static + Error + Send + Sync;
    type EventData: 'static + Send + Sync;
    type MetaData: 'static + Send + Sync;

    /// Called by eventific as part of the setup process
    ///
    /// All your setup such as setting up connection pools, verify that targets exists, or other stuff should be
    /// implemented here
    async fn init(&mut self, context: StoreContext) -> Result<(), Self::Error>;

    async fn save_events(
        &self,
        context: StoreContext,
        events: &Vec<Event<Self::EventData, Self::MetaData>>,
    ) -> Result<SaveEventsResult, Self::Error>;

    /// Events returned from this stream has to be in the correct order
    async fn events(
        &self,
        context: StoreContext,
        aggregate_id: Uuid,
    ) -> Result<BoxStream<'_, Result<Event<Self::EventData, Self::MetaData>, Self::Error>>, Self::Error>;

    /// Gets all aggregate ids
    async fn aggregate_ids(
        &self,
        context: StoreContext,
    ) -> Result<BoxStream<'_, Result<Uuid, Self::Error>>, Self::Error>;

    /// This should return the total number of aggregates in the store
    async fn total_aggregates(
        &self,
        context: StoreContext,
    ) -> Result<u64, Self::Error> {
        Ok(self
            .aggregate_ids(context)
            .await?
            .try_fold(0, |acc, _id| async move { Ok(acc + 1 as u64) })
            .await?)
    }

    /// This method should return the total number of events in the store for a single aggregate
    async fn total_events_for_aggregate(
        &self,
        context: StoreContext,
        aggregate_id: Uuid,
    ) -> Result<u64, Self::Error> {
        Ok(self
            .events(context, aggregate_id)
            .await?
            .try_fold(0, |acc, _event| async move { Ok(acc + 1 as u64) })
            .await?)
    }

    /// This method should return the total number of events in the store, all aggregates combined
    async fn total_events(
        &self,
        context: StoreContext,
    ) -> Result<u64, Self::Error> {
        let id_stream = self.aggregate_ids(context.clone()).await?;
        let total_events = id_stream
            .try_fold(0 as u64, |acc, id| {
                let context = context.clone();
                async move {
                    let events: Vec<_> = self.events(context, id).await?.collect().await;

                    Ok(events.len() as u64 + acc)
                }
            })
            .await?;

        Ok(total_events)
    }
}
