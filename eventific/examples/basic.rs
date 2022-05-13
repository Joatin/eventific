use eventific::State;
use eventific::{Event, EventStoreBuilder};
use uuid::Uuid;

#[derive(Clone, Debug)]
enum MyEvent {
    FirstEvent,
    SecondEvent,
}

#[derive(Default, Debug)]
struct MyState {
    num_events: usize,
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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // SETUP
    let event_store = EventStoreBuilder::new().build_with_memory_storage();

    // Get an aggregate
    let aggregate = event_store.aggregate(Uuid::nil());

    // Store some events
    aggregate
        .save_events(|_state: MyState| Ok(vec![MyEvent::FirstEvent {}, MyEvent::SecondEvent {}]))
        .await?;

    // Load the state
    let _ = aggregate.state::<MyState>().await?;

    Ok(())
}
