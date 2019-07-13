use futures::Stream;
use uuid::Uuid;

pub trait Listener {
    fn listen(&self) -> Box<Stream<Item = Uuid, Error = ()>>;
}
