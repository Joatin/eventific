use uuid::Uuid;

#[derive(Debug, Default)]
pub struct AddEventsParams<M> {
    pub aggregate_id: Uuid,
    pub metadata: Option<M>,
}
