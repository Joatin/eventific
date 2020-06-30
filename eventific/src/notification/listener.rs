use uuid::Uuid;
use crate::notification::NotificationError;
use slog::Logger;
use futures::future::BoxFuture;
use futures::stream::BoxStream;

pub trait Listener: Send + Sync {
    fn init<'a>(&'a mut self, logger: &'a Logger, service_name: &'a str) -> BoxFuture<'a, Result<(), NotificationError>>;
    fn listen<'a>(&'a self, logger: &'a Logger) -> BoxFuture<'a, Result<BoxStream<'a, Result<Uuid, NotificationError>>, NotificationError>>;
}
