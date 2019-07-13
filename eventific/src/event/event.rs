use uuid::Uuid;
use chrono::DateTime;
use chrono::Utc;
use std::collections::HashMap;
use std::fmt::{Display, Debug, Formatter};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Event<D> {
    pub aggregate_id: Uuid,
    pub event_id: u32,
    pub created_date: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub payload: D
}
