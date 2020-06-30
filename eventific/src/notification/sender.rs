use uuid::Uuid;
use crate::notification::NotificationError;
use slog::Logger;
use futures::future::BoxFuture;

pub trait Sender: Send + Sync {
    fn init<'a>(&'a mut self, logger: &'a Logger, service_name: &'a str) -> BoxFuture<'a, Result<(), NotificationError>>;
    fn send<'a>(&'a self, logger: &'a Logger, aggregate_id: Uuid) -> BoxFuture<'a, Result<(), NotificationError>>;
}
