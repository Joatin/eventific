extern crate eventific;
extern crate futures;
extern crate tokio;
extern crate sloggers;
#[macro_use]
extern crate slog;

use eventific::{EventificBuilder, Eventific};
use futures::future::Future;
use eventific::store::MemoryStore;
use sloggers::terminal::TerminalLoggerBuilder;
use sloggers::Build;
use uuid::Uuid;

#[derive(Default, Debug)]
struct SimpleState;

#[derive(Debug, Clone)]
enum EventData {
    TitleChanged(String)
}

/// This example showcases how you can use eventific to store and retrieve events. In a real world use case this would
/// probably not happen in the same service, you would instead have one service for persisting and another for reading
fn main() {
    let logger = TerminalLoggerBuilder::new().build().unwrap();

    let run_future = EventificBuilder::new()
        .logger(&logger)
        .start()
        .and_then(move |eventific: Eventific<MemoryStore<EventData>, EventData, SimpleState>| {
            let id = Uuid::nil();
            eventific.create_aggregate(id, vec![EventData::TitleChanged("HelloWorld".to_owned())], None)
                .and_then(move |()| {
                    eventific.aggregate(id)
                        .and_then(move |aggregate| {
                            info!(logger, "{:#?}", aggregate);
                            Ok(())
                        })
                })
        })
        .map_err(|err| eprintln!("{}", err));

    // We always start eventific by scheduling on a executor. Tokio is one of the simplest implementations
    tokio::run(run_future);
}
