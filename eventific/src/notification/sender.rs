use uuid::Uuid;
use futures::Future;

pub trait Sender {
    fn send(&self, aggregate_id: Uuid) -> Box<Future<Item = (), Error = ()>>;
}
