use futures::{Stream, Future};
use uuid::Uuid;
use crate::notification::NotificationError;
use slog::Logger;

pub trait Listener: Send + Sync {
    fn init(&mut self, logger: &Logger, service_name: &str) -> Box<Future<Item = (), Error = NotificationError> + Send>;
    fn listen(&self) -> Box<Stream<Item = Uuid, Error = NotificationError> + Send>;
}
