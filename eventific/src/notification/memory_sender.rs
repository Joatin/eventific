use crate::notification::{Sender, NotificationError};
use uuid::Uuid;
use futures::Future;
use futures::sink::Sink;
use std::sync::{Arc, Mutex};
use futures::sync::mpsc;
use slog::Logger;

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
    fn init(&mut self, logger: &Logger, _service_name: &str) -> Box<Future<Item=(), Error=NotificationError> + Send> {
        info!(logger, "🧠  Creating new MemorySender");
        warn!(logger, "🚨  This sender will send new notifications to a local runtime bound queue. This will only work with a single process and is therefor not suited for clustered or production environments");
        Box::new(futures::finished(()))
    }

    fn send(&self, aggregate_id: Uuid) -> Box<Future<Item=(), Error=NotificationError> + Send> {
        let senders: Vec<_> = {
            let lock = self.listeners.lock().unwrap();
            lock.iter().cloned().collect()
        };

        let result_future = futures::future::join_all(senders.into_iter().map(move |send| {
            send.send(aggregate_id)
        }))
            .map(|_|())
            .map_err(|err| NotificationError::FailedToSend(format_err!("{}", err)));

        Box::new(result_future)
    }
}
