use slog::Logger;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct CreateAggregateParams<D, M> {
    pub aggregate_id: Uuid,
    pub events: Vec<D>,
    pub metadata: Option<M>,
    pub logger: Option<Logger>,
}
