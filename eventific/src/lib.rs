#![no_std]

extern crate alloc;

mod aggregate;
mod event;
mod event_store;
mod event_store_builder;
mod memory_storage;
mod notifier;
mod save_events_result;
mod state;
mod storage;

pub use self::aggregate::Aggregate;
pub use self::event::Event;
pub use self::event_store::EventStore;
pub use self::event_store_builder::EventStoreBuilder;
pub use self::notifier::Notifier;
pub use self::save_events_result::SaveEventsResult;
pub use self::state::State;
pub use self::storage::Storage;

pub use uuid::Uuid;
