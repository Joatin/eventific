use crate::notification::local_sender::LocalSender;
use crate::notification::local_receiver::LocalReceiver;
use tokio::sync::broadcast;

pub fn create_local_sender_receiver() -> (LocalSender, LocalReceiver) {
    let (send, _) = broadcast::channel(1024);

    (
        LocalSender::new(send.clone()),
        LocalReceiver::new(send)
    )
}
