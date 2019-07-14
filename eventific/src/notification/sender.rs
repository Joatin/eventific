use uuid::Uuid;
use futures::Future;
use crate::notification::NotificationError;
use slog::Logger;

pub trait Sender: Send + Sync {
    fn init(&mut self, logger: &Logger, service_name: &str) -> Box<Future<Item = (), Error = NotificationError> + Send>;
    fn send(&self, aggregate_id: Uuid) -> Box<Future<Item = (), Error = NotificationError> + Send>;
}
