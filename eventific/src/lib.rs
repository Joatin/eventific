#![no_std]

extern crate alloc;

mod event;
mod aggregate;
mod event_store;
mod storage;
mod event_store_builder;
mod memory_storage;
mod state;
mod notifier;
mod save_events_result;

pub use self::event_store_builder::EventStoreBuilder;
pub use self::event_store::EventStore;
pub use self::state::State;
pub use self::event::Event;
pub use self::storage::Storage;
pub use self::save_events_result::SaveEventsResult;
pub use self::aggregate::Aggregate;
pub use self::notifier::Notifier;

pub use uuid::Uuid;
