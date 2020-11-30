use chrono::DateTime;
use chrono::Utc;
use std::fmt::Debug;
use strum::IntoEnumIterator;
use uuid::Uuid;

/// A event that is stored in the store
///
/// This is one of the most central constructs in eventific. The event is used to build the aggregates. Remember that
/// states are the "source of truth", this means that they can never change. That's why Eventific don't have any means
/// to delete or update your events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event<D, M> {
    /// The Id of the aggregate
    pub aggregate_id: Uuid,
    /// This events Id, this is a incremental number starting from 0
    pub event_id: u32,
    /// The date this event was created
    pub created_date: DateTime<Utc>,
    /// Additional metadata
    pub metadata: Option<M>,
    /// The events payload
    pub payload: D,
}
