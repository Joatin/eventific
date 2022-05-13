use alloc::boxed::Box;
use futures::stream::BoxStream;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait Notifier<P: Send + Sync>: Send + Sync {
    async fn send_event(&self, aggregate_id: Uuid, event_id: u64, payload: P)
        -> anyhow::Result<()>;
    async fn on_event<'a>(&self) -> anyhow::Result<BoxStream<'a, (Uuid, u64, Option<P>)>>;
}
