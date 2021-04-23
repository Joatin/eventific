

#[derive(
Debug,
Clone,
strum_macros::EnumIter,
strum_macros::AsRefStr,
serde::Serialize,
serde::Deserialize,
)]
pub enum TestEventData {
    Event1,
    Event2,
    Event3,
}
