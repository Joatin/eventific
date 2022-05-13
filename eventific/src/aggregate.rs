use crate::event::Event;
use crate::state::State;
use crate::storage::Storage;
use crate::SaveEventsResult;
use alloc::sync::Arc;
use alloc::vec::Vec;
use futures::stream::BoxStream;
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;

/// An aggregate root. Use this struct to access any state or events that belongs to it.
///
/// This struct is safe to clone.
pub struct Aggregate<P> {
    id: Uuid,
    storage: Arc<dyn Storage<P>>,
}

impl<P: Send + Sync> Aggregate<P> {
    pub(crate) fn new(id: Uuid, storage: Arc<dyn Storage<P>>) -> Self {
        Self { id, storage }
    }

    /// Returns the id of the aggregate
    ///
    /// # Examples
    ///
    /// ```
    /// # use eventific::{EventStoreBuilder, EventStore};
    /// # use uuid::Uuid;
    /// #
    /// # #[derive(Clone)]
    /// # struct Payload;
    /// #
    /// # let event_store: EventStore<Payload> = EventStoreBuilder::new().build_with_memory_storage();
    /// # let aggregate = event_store.aggregate(Uuid::nil());
    /// #
    /// let id = aggregate.id();
    /// ```
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns the current state for the aggregate
    ///
    /// # Examples
    ///
    /// ```
    /// # use eventific::{EventStoreBuilder, EventStore, State, Event};
    /// # use uuid::Uuid;
    /// # use std::error::Error;
    /// #
    /// # #[derive(Clone)]
    /// # struct Payload;
    /// #
    /// # #[derive(Default)]
    /// # struct MyState;
    /// #
    /// # impl State<Payload> for MyState {
    /// #    fn apply(&mut self, event: Event<Payload>) {
    /// #        todo!()
    /// #    }
    /// #
    /// # }
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), anyhow::Error> {
    /// #     let event_store: EventStore<Payload> = EventStoreBuilder::new().build_with_memory_storage();
    /// #     let aggregate = event_store.aggregate(Uuid::nil());
    /// #
    ///       let state: MyState = aggregate.state().await?;
    ///       Ok(())
    /// }
    /// ```
    pub async fn state<S: State<P>>(&self) -> anyhow::Result<S> {
        let event_stream = self.events().await;
        let state = event_stream
            .try_fold(S::default(), |mut acc, event| async move {
                acc.apply(event);
                Ok(acc)
            })
            .await?;
        Ok(state)
    }

    /// Returns a stream of events from this aggregat
    pub async fn events(&self) -> BoxStream<'_, anyhow::Result<Event<P>>> {
        self.storage
            .events_for_aggregate(&self.id)
            .await
            .map_ok(move |(id, payload)| Event::new(self.clone(), id, payload))
            .boxed()
    }

    /// The total events stored in this aggregate. A value of 0 means none
    pub async fn total_events(&self) -> anyhow::Result<u64> {
        self.storage.total_events_for_aggregate(&self.id).await
    }

    /// The current version of this aggregate. 0 means that it hasn't been created yet
    pub async fn version(&self) -> anyhow::Result<u64> {
        self.storage.aggregate_version(self.id()).await
    }

    /// Stores some events to the aggregate root. Since another instance might push events before us, the callback can
    /// be called several times. Since the state changes everytime the callback is rerun, it's important to do all
    /// validation within the callback and not before
    pub async fn save_events<CB: Fn(S) -> anyhow::Result<Vec<P>>, S: State<P>>(
        &self,
        event_callback: CB,
    ) -> anyhow::Result<()> {
        loop {
            let current_version = self.version().await?;
            let state = self.state().await?;
            let events = event_callback(state)?;
            let events = events
                .into_iter()
                .enumerate()
                .map(|(index, payload)| (current_version + index as u64 + 1, payload))
                .collect::<Vec<_>>();

            match self.storage.save_events(self.id(), events).await {
                SaveEventsResult::Ok => {
                    log::info!("Saved events for aggregate '{}'", self.id());
                    break;
                }
                SaveEventsResult::VersionConflict => {
                    log::info!("Someone else saved events before us while saving events for aggregate '{}', retrying...", self.id());
                    continue;
                }
                SaveEventsResult::Error(error) => return Err(error),
            }
        }
        Ok(())
    }
}

impl<P> Clone for Aggregate<P> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            storage: Arc::clone(&self.storage),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::aggregate::Aggregate;
    use crate::storage::test::MockStorage;
    use alloc::sync::Arc;
    use uuid::Uuid;

    struct Payload;

    #[test]
    fn it_should_return_correct_id() {
        let storage = MockStorage::default();
        let id = Uuid::new_v4();
        let aggregate = Aggregate::<Payload>::new(id, Arc::new(storage));

        assert_eq!(aggregate.id(), &id)
    }
}
