use uuid::Uuid;
use crate::notification::{Listener, NotificationError};
use futures::{future, FutureExt};
use std::sync::{Mutex, Arc};
use futures::future::{BoxFuture};
use slog::Logger;
use futures::stream::BoxStream;
use futures::channel::mpsc;
use futures::StreamExt;


pub struct MemoryListener {
    listeners: Arc<Mutex<Vec<mpsc::Sender<Uuid>>>>
}

impl MemoryListener {
    pub(crate) fn new(listeners: Arc<Mutex<Vec<mpsc::Sender<Uuid>>>>) -> Self {
        Self {
            listeners
        }
    }
}

impl Listener for MemoryListener {
    fn init<'a>(&'a mut self, logger: &'a Logger, _service_name: &'a str) -> BoxFuture<'a, Result<(), NotificationError>> {
        info!(logger, "🧠  Creating new MemoryListener");
        warn!(logger, "🚨  This listener will listen for new notifications in a local runtime bound queue. This will only work with a single process and is therefor not suited for clustered or production environments");

        future::ok(()).boxed()
    }

    fn listen<'a>(&'a self, _logger: &'a Logger) -> BoxFuture<'a, Result<BoxStream<'a, Result<Uuid, NotificationError>>, NotificationError>> {
        async move {
            let (sender, receiver) = mpsc::channel::<Uuid>(10000);
            {
                let mut lock = self.listeners.lock().unwrap();
                lock.push(sender);
            }
            let stream: BoxStream<_> = receiver
                .then(|i| future::ok(i))
                .boxed();

            Ok(stream)
        }.boxed()
    }
}
