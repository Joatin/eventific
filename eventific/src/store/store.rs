use crate::event::{Event, EventData};
use futures::future::BoxFuture;
use futures::stream::{BoxStream, TryStreamExt};
use crate::store::StoreError;
use uuid::Uuid;
use slog::Logger;
use futures::{StreamExt, FutureExt};

pub trait Store<D: EventData>: 'static + Send + Clone + Sync {

    /// Called by eventific as part of the setup process
    fn init<'a>(&'a mut self, logger: &'a Logger, service_name: &str) -> BoxFuture<'a, Result<(), StoreError<D>>>;

    fn save_events<'a>(
        &'a self,
        logger: &'a Logger,
        events: Vec<Event<D>>
    ) -> BoxFuture<'a, Result<(), StoreError<D>>>;

    /// Events returned from this stream has to be in the correct order
    fn events<'a>(
        &'a self,
        logger: &'a Logger,
        aggregate_id: Uuid,
    ) -> BoxFuture<'a, Result<BoxStream<'a, Result<Event<D>, StoreError<D>>>, StoreError<D>>>;

    /// Gets all aggregate ids
    fn aggregate_ids<'a>(
        &'a self,
        logger: &'a Logger,
    ) -> BoxFuture<'a, Result<BoxStream<'a, Result<Uuid, StoreError<D>>>, StoreError<D>>>;

    fn total_aggregates<'a>(
        &'a self,
        logger: &'a Logger,
    ) -> BoxFuture<'a, Result<u64, StoreError<D>>> {
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
    ) -> BoxFuture<'a, Result<u64, StoreError<D>>> {
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
    ) -> BoxFuture<'a, Result<u64, StoreError<D>>> {
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
