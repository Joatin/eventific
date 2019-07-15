use crate::notification::memory_sender::MemorySender;
use crate::notification::memory_listener::MemoryListener;
use futures::sync::mpsc::Sender;
use uuid::Uuid;
use std::sync::{Mutex, Arc};

pub fn create_memory_notification_pair() -> (MemorySender, MemoryListener) {

    let listeners: Arc<Mutex<Vec<Sender<Uuid>>>> = Arc::new(Mutex::new(Vec::new()));

    let memory_sender = MemorySender::new(Arc::clone(&listeners));
    let memory_listener = MemoryListener::new(listeners);

    (memory_sender, memory_listener)
}
