extern crate eventific;
extern crate futures;
extern crate tokio;
extern crate sloggers;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate strum_macros;

use eventific::{EventificBuilder, Eventific};
use futures::future::Future;
use eventific::store::MemoryStore;
use sloggers::terminal::TerminalLoggerBuilder;
use sloggers::Build;
use uuid::Uuid;
use futures::Stream;
use sloggers::types::Format;

#[derive(Default, Debug)]
struct SimpleState;

#[derive(Debug, Clone, EnumIter, AsRefStr)]
enum EventData {
    TitleChanged(String)
}

/// This example showcases how you can use eventific to store and retrieve events. In a real world use case this would
/// probably not happen in the same service, you would instead have one service for persisting and another for reading
fn main() {
    let logger = TerminalLoggerBuilder::new().format(Format::Compact).build().unwrap();

    let run_future = EventificBuilder::new()
        .logger(&logger)
        .enable_playground()
        .start()
        .and_then(move |eventific: Eventific<SimpleState, EventData>| {
            futures::future::join_all(vec![
                eventific.create_aggregate(Uuid::new_v4(), vec![EventData::TitleChanged("HelloWorld".to_owned())], None),
                eventific.create_aggregate(Uuid::new_v4(), vec![EventData::TitleChanged("HelloWorld".to_owned())], None),
                eventific.create_aggregate(Uuid::new_v4(), vec![EventData::TitleChanged("HelloWorld".to_owned())], None),
            ])
                .map(|_|())
        })
        .map_err(|err| eprintln!("{}", err));

    // We always start eventific by scheduling on a executor. Tokio is one of the simplest implementations
    tokio::run(run_future);
}
