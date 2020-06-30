use crate::notification::{Sender, NotificationError};
use uuid::Uuid;
use futures::{future, FutureExt, SinkExt};
use std::sync::{Arc, Mutex};
use futures::channel::mpsc;
use slog::Logger;
use futures::future::BoxFuture;


pub struct MemorySender {
    listeners: Arc<Mutex<Vec<mpsc::Sender<Uuid>>>>
}

impl MemorySender {
    pub(crate) fn new(listeners: Arc<Mutex<Vec<mpsc::Sender<Uuid>>>>) -> Self {
        Self {
            listeners
        }
    }
}

impl Sender for MemorySender {
    fn init<'a>(&'a mut self, logger: &'a Logger, _service_name: &'a str) -> BoxFuture<'a, Result<(), NotificationError>> {
        info!(logger, "🧠  Creating new MemorySender");
        warn!(logger, "🚨  This sender will send new notifications to a local runtime bound queue. This will only work with a single process and is therefor not suited for clustered or production environments");
        future::ok(()).boxed()
    }

    fn send<'a>(&'a self, _logger: &'a Logger, aggregate_id: Uuid) -> BoxFuture<'a, Result<(), NotificationError>> {
        async move {
            let senders: Vec<_> = {
                let lock = self.listeners.lock().unwrap();
                lock.iter().cloned().collect()
            };

            for mut sender in senders {
                sender.send(aggregate_id)
                    .await
                    .map_err(|err| NotificationError::FailedToSend(format_err!("{}", err)))?;
            }

            Ok(())
        }.boxed()
    }
}
