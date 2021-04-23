use crate::store::Store;
use std::fmt::Debug;
use strum::IntoEnumIterator;
use crate::notification::Receiver;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::{ Receiver as TokioReceiver };
use crate::{Uuid, Eventific};
use std::error::Error;

#[derive(Debug)]
pub struct LocalReceiver {
    sender: Sender<Uuid>
}

impl LocalReceiver {
    pub(in crate::notification) fn new(sender: Sender<Uuid>) -> Self {
        Self {
            sender
        }
    }
}

#[async_trait::async_trait]
impl<
    St: Store<EventData = D, MetaData = M>,
    S: Send,
    D: 'static + Debug + Clone + Send + Sync + IntoEnumIterator,
    M: 'static + Send + Sync + Debug,
> Receiver<St, S, D, M> for LocalReceiver {
    async fn init(&mut self, eventific: &Eventific<St, S, D, M>, sender: Sender<Uuid>) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut recv = self.sender.subscribe();
        tokio::spawn(async move {
            while let id = recv.recv().await.unwrap() {
                let _res = sender.send(id);
            }
        });

        Ok(())
    }

    fn name(&self) -> &str {
        "LocalReceiver 🙈"
    }
}
