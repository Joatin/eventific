use eventific::store::{Store, StoreError};
use futures::{Future, Stream};
use eventific::event::Event;
use uuid::Uuid;
use slog::Logger;

pub struct DynamoStore {}

impl DynamoStore {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl<D> Store<D> for DynamoStore {
    fn init(&mut self, logger: &Logger, service_name: &str) -> Box<Future<Item=(), Error=StoreError<D>>> {
        unimplemented!()
    }

    fn save_events(&self, events: Vec<Event<D>>) -> Box<Future<Item=(), Error=StoreError<D>>> {
        unimplemented!()
    }

    fn events(&self, aggregate_id: Uuid) -> Box<Future<Item=Vec<Event<D>>, Error=StoreError<D>>> {
        unimplemented!()
    }

    fn aggregate_ids(&self) -> Box<Stream<Item=Uuid, Error=StoreError<D>>> {
        unimplemented!()
    }

    fn total_aggregates(&self) -> Box<Future<Item=u64, Error=StoreError<D>>> {
        unimplemented!()
    }

    fn total_events_for_aggregate(&self, aggregate_id: Uuid) -> Box<Future<Item=u64, Error=StoreError<D>>> {
        unimplemented!()
    }

    fn total_events(&self) -> Box<Future<Item=u64, Error=StoreError<D>>> {
        unimplemented!()
    }
}
