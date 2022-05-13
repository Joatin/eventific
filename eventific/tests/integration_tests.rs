use eventific::{EventStoreBuilder, Event};
use eventific::State;
use uuid::Uuid;

#[derive(Clone, Debug)]
enum MyEvent {
    FirstEvent,
    SecondEvent,
}

#[derive(Default, Debug)]
struct MyState {
    pub num_events: usize
}

impl State<MyEvent> for MyState {
    fn apply(&mut self, event: Event<MyEvent>) {
        match event.payload() {
            MyEvent::FirstEvent | MyEvent::SecondEvent => {
                self.num_events += 1;
            }
        }
    }
}

#[tokio::test]
async fn it_should_save_events() -> Result<(), anyhow::Error> {
    let event_store = EventStoreBuilder::new().build_with_memory_storage();
    let aggregate = event_store.aggregate(Uuid::nil());

    assert_eq!(aggregate.total_events().await?, 0);

    // Store some events
    aggregate.save_events(|_state: MyState| {
        Ok(vec![
            MyEvent::FirstEvent {},
            MyEvent::SecondEvent {},
        ])
    }).await?;

    assert_eq!(aggregate.total_events().await?, 2);

    Ok(())
}

#[tokio::test]
async fn it_should_apply_events_to_state() -> Result<(), anyhow::Error> {
    let event_store = EventStoreBuilder::new().build_with_memory_storage();
    let aggregate = event_store.aggregate(Uuid::nil());

    // Store some events
    aggregate.save_events(|_state: MyState| {
        Ok(vec![
            MyEvent::FirstEvent {},
            MyEvent::SecondEvent {},
        ])
    }).await?;

    assert_eq!(aggregate.state::<MyState>().await?.num_events, 2);

    Ok(())
}
