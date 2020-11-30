use crate::event::Event;
use crate::store::store_context::StoreContext;
use crate::store::SaveEventsResult;
use futures::future::BoxFuture;
use futures::stream::{BoxStream, TryStreamExt};
use futures::{FutureExt, StreamExt};
use slog::Logger;
use std::error::Error;
use std::fmt::Debug;
use strum::IntoEnumIterator;
use uuid::Uuid;

/// This trait is used for all component that wish to act as an event store
pub trait Store: 'static + Send + Sync {
    type Error: 'static + Error + Send + Sync;
    type EventData: 'static + Send + Sync;
    type MetaData: 'static + Send + Sync;

    /// Called by eventific as part of the setup process
    ///
    /// All your setup such as setting up connection pools, verify that targets exists, or other stuff should be
    /// implemented here
    fn init<'a>(&'a mut self, context: StoreContext) -> BoxFuture<'a, Result<(), Self::Error>>;

    fn save_events<'a>(
        &'a self,
        context: StoreContext,
        events: &'a Vec<Event<Self::EventData, Self::MetaData>>,
    ) -> BoxFuture<'a, Result<SaveEventsResult, Self::Error>>;

    /// Events returned from this stream has to be in the correct order
    fn events<'a>(
        &'a self,
        context: StoreContext,
        aggregate_id: Uuid,
    ) -> BoxFuture<
        'a,
        Result<
            BoxStream<'a, Result<Event<Self::EventData, Self::MetaData>, Self::Error>>,
            Self::Error,
        >,
    >;

    /// Gets all aggregate ids
    fn aggregate_ids<'a>(
        &'a self,
        context: StoreContext,
    ) -> BoxFuture<'a, Result<BoxStream<'a, Result<Uuid, Self::Error>>, Self::Error>>;

    /// This should return the total number of aggregates in the store
    fn total_aggregates<'a>(
        &'a self,
        context: StoreContext,
    ) -> BoxFuture<'a, Result<u64, Self::Error>> {
        async move {
            Ok(self
                .aggregate_ids(context)
                .await?
                .try_fold(0, |acc, _id| async move { Ok(acc + 1 as u64) })
                .await?)
        }
        .boxed()
    }

    /// This method should return the total number of events in the store for a single aggregate
    fn total_events_for_aggregate<'a>(
        &'a self,
        context: StoreContext,
        aggregate_id: Uuid,
    ) -> BoxFuture<'a, Result<u64, Self::Error>> {
        async move {
            Ok(self
                .events(context, aggregate_id)
                .await?
                .try_fold(0, |acc, _event| async move { Ok(acc + 1 as u64) })
                .await?)
        }
        .boxed()
    }

    /// This method should return the total number of events in the store, all aggregates combined
    fn total_events<'a>(
        &'a self,
        context: StoreContext,
    ) -> BoxFuture<'a, Result<u64, Self::Error>> {
        async move {
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
        .boxed()
    }
}
