use crate::store::Store;
use std::fmt::Debug;
use strum::IntoEnumIterator;
use crate::notification::Sender;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::{ Sender as TokioSender };
use crate::{Uuid, Eventific};
use std::error::Error;

#[derive(Debug)]
pub struct LocalSender {
    sender: TokioSender<Uuid>
}

impl LocalSender {
    pub(in crate::notification) fn new(sender: TokioSender<Uuid>) -> Self {
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
> Sender<St, S, D, M> for LocalSender {
    async fn init(&mut self, eventific: &Eventific<St, S, D, M>, mut receiver: Receiver<Uuid>) -> Result<(), Box<dyn Error + Send + Sync>> {

        let sender = self.sender.clone();
        tokio::spawn(async move {
            while let id = receiver.recv().await.unwrap() {
                let _res = sender.send(id);
            }
        });

        Ok(())
    }

    fn name(&self) -> &str {
        "LocalSender 🙉"
    }
}
