use slog::Logger;
use uuid::Uuid;
use std::fmt::Debug;

#[derive(Debug)]
pub struct CreateAggregateParams<D, M> {
    pub aggregate_id: Uuid,
    pub events: Vec<D>,
    pub metadata: Option<M>,
    pub logger: Option<Logger>,
}

impl<D, M> Default for CreateAggregateParams<D, M> {
    fn default() -> Self {
        Self {
            aggregate_id: Uuid::nil(),
            events: vec![],
            metadata: None,
            logger: None
        }
    }
}
