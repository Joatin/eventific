use crate::event::{Event, EventData};
use futures::future::BoxFuture;
use futures::stream::{BoxStream, TryStreamExt};
use crate::store::StoreError;
use uuid::Uuid;
use slog::Logger;
use futures::{StreamExt, FutureExt};
use std::fmt::Debug;

pub trait Store<D: EventData, M: 'static + Send + Sync + Debug>: 'static + Send + Clone + Sync {

    /// Called by eventific as part of the setup process
    fn init<'a>(&'a mut self, logger: &'a Logger, service_name: &str) -> BoxFuture<'a, Result<(), StoreError<D, M>>>;

    fn save_events<'a>(
        &'a self,
        logger: &'a Logger,
        events: Vec<Event<D, M>>
    ) -> BoxFuture<'a, Result<(), StoreError<D, M>>>;

    /// Events returned from this stream has to be in the correct order
    fn events<'a>(
        &'a self,
        logger: &'a Logger,
        aggregate_id: Uuid,
    ) -> BoxFuture<'a, Result<BoxStream<'a, Result<Event<D, M>, StoreError<D, M>>>, StoreError<D, M>>>;

    /// Gets all aggregate ids
    fn aggregate_ids<'a>(
        &'a self,
        logger: &'a Logger,
    ) -> BoxFuture<'a, Result<BoxStream<'a, Result<Uuid, StoreError<D, M>>>, StoreError<D, M>>>;

    fn total_aggregates<'a>(
        &'a self,
        logger: &'a Logger,
    ) -> BoxFuture<'a, Result<u64, StoreError<D, M>>> {
        async move {
            Ok(self.aggregate_ids(&logger)
                .await?
                .try_fold(0, |acc, _id| {
                    async move { Ok(acc + 1 as u64) }
                })
                .await?)
        }.boxed()
    }

    fn total_events_for_aggregate<'a>(
        &'a self,
        logger: &'a Logger,
        aggregate_id: Uuid,
    ) -> BoxFuture<'a, Result<u64, StoreError<D, M>>> {
        async move {
            Ok(self.events(&logger, aggregate_id)
                .await?
                .try_fold(0, |acc, _event| {
                    async move { Ok(acc + 1 as u64) }
                })
                .await?
            )
        }.boxed()
    }

    fn total_events<'a>(
        &'a self,
        logger: &'a Logger,
    ) -> BoxFuture<'a, Result<u64, StoreError<D, M>>> {
        async move {
            let id_stream = self.aggregate_ids(&logger).await?;

            let total_events  = id_stream.try_fold(0 as u64, |acc, id| async move {
                    let events: Vec<_> = self.events(&logger, id)
                        .await?
                        .collect().await;

                    Ok(events.len() as u64 + acc)
                }).await?;

            Ok(total_events)
        }.boxed()
    }

}
