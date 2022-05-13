use crate::aggregate::Aggregate;

/// Wraps the event payload and add some common attributes
///
/// The payload can be of any type, but it's most commonly an enum that describes different type of events that has
/// happened. Eventific puts no requirements on the payload type, however, different storages might require some
/// additional traits.
pub struct Event<P> {
    aggregate: Aggregate<P>,
    id: u64,
    payload: P,
}

impl<P> Event<P> {
    pub(crate) fn new(aggregate: Aggregate<P>, id: u64, payload: P) -> Self {
        Self {
            aggregate,
            id,
            payload,
        }
    }

    /// Return the id of of the event. The id is an incremental number starting at 1, and increases for each added
    /// event to the aggregate root.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns a reference to the aggregate this event belongs to
    pub fn aggregate(&self) -> &Aggregate<P> {
        &self.aggregate
    }

    /// The payload of this event
    pub fn payload(&self) -> &P {
        &self.payload
    }
}

#[cfg(test)]
mod tests {
    use crate::Event;

    use crate::aggregate::*;
    use crate::storage::test::MockStorage;
    use alloc::sync::Arc;
    use uuid::Uuid;

    enum MyPayload {
        Created,
    }

    #[test]
    fn it_should_return_correct_id() {
        let storage = MockStorage::default();
        let aggregate = Aggregate::new(Uuid::nil(), Arc::new(storage));

        let event = Event::new(aggregate, 123, MyPayload::Created);

        assert_eq!(event.id(), 123)
    }
}
