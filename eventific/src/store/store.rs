use crate::event::Event;
use futures::Future;
use futures::stream::Stream;
use crate::store::StoreError;
use uuid::Uuid;
use slog::Logger;
use std::fmt::Debug;

pub trait Store<D: 'static + Send + Sync + Debug>: 'static + Send + Clone {

    /// Called by eventific as part of the setup process
    fn init(&mut self, logger: &Logger, service_name: &str) -> Box<Future<Item = (), Error = StoreError<D>> + Send>;

    fn save_events(
        &self,
        events: Vec<Event<D>>
    ) -> Box<Future<Item = (), Error = StoreError<D>> + Send>;

    fn events(
        &self,
        aggregate_id: Uuid,
    ) -> Box<Future<Item = Vec<Event<D>>, Error = StoreError<D>> + Send>;

    /// Gets all aggregate ids
    fn aggregate_ids(
        &self
    ) -> Box<Stream<Item = Uuid, Error = StoreError<D>> + Send>;

    fn total_aggregates(
        &self,
    ) -> Box<Future<Item = u64, Error = StoreError<D>> + Send> {
        let res = self.aggregate_ids()
            .collect()
            .map(|ids| ids.len() as u64);

        Box::new(res)
    }

    fn total_events_for_aggregate(
        &self,
        aggregate_id: Uuid,
    ) -> Box<Future<Item = u64, Error = StoreError<D>> + Send> {
        let res = self.events(aggregate_id)
            .map(|events| events.len() as u64);

        Box::new(res)
    }

    fn total_events(
        &self
    ) -> Box<Future<Item = u64, Error = StoreError<D>> + Send> {
        let self_clone = self.clone();
        let res = self.aggregate_ids()
            .and_then(move |id| {
                self_clone.events(id)
                    .and_then(|events| Ok(events.len() as u64))
            })
            .collect()
            .map(|event_lengths| {
                event_lengths.iter().sum()
            });

        Box::new(res)
    }

}
