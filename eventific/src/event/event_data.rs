use std::fmt::Debug;


/// Marker trait for event data
pub trait EventData: 'static + Debug + Clone + Send + Sync {}
