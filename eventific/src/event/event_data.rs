use std::fmt::Debug;
use strum::IntoEnumIterator;


/// Marker trait for event data
pub trait EventData: 'static + Debug + Clone + Send + Sync + IntoEnumIterator {}
