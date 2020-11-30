use slog::Logger;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct AddEventsParams<M> {
    pub aggregate_id: Uuid,
    pub logger: Option<Logger>,
    pub metadata: Option<M>,
}
