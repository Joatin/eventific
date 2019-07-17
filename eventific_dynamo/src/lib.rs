
#[macro_use]
extern crate failure;

mod dynamo_store;

pub use self::dynamo_store::DynamoStore;
pub use rusoto_core::Region;
