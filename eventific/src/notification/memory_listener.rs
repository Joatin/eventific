use futures::sync::mpsc::{channel, Sender};
use uuid::Uuid;
use crate::notification::{Listener, NotificationError};
use futures::Stream;
use std::sync::{Mutex, Arc};
use futures::future::Future;
use slog::Logger;

pub struct MemoryListener {
    listeners: Arc<Mutex<Vec<Sender<Uuid>>>>
}

impl MemoryListener {
    pub(crate) fn new(listeners: Arc<Mutex<Vec<Sender<Uuid>>>>) -> Self {
        Self {
            listeners
        }
    }
}

impl Listener for MemoryListener {
    fn init(&mut self, logger: &Logger, _service_name: &str) -> Box<Future<Item=(), Error=NotificationError> + Send> {
        info!(logger, "🧠  Creating new MemoryListener");
        warn!(logger, "🚨  This listener will listen for new notifications in a local runtime bound queue. This will only work with a single process and is therefor not suited for clustered or production environments");

        Box::new(futures::finished(()))
    }

    fn listen(&self) -> Box<Stream<Item=Uuid, Error=NotificationError> + Send> {
        let (sender, receiver) = channel::<Uuid>(10000);
        {
            let mut lock = self.listeners.lock().unwrap();
            lock.push(sender);
        }
        let result_stream = receiver.map_err(|_| NotificationError::FailedToListen(format_err!("This error can't happen")));

        Box::new(result_stream)
    }
}
