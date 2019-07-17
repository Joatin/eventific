extern crate eventific;
extern crate futures;
extern crate tokio;
extern crate sloggers;
#[macro_use]
extern crate slog;
extern crate grpcio;
extern crate protobuf;

use eventific::{EventificBuilder, Eventific};
use futures::future::Future;
use eventific::store::MemoryStore;
use sloggers::terminal::TerminalLoggerBuilder;
use sloggers::Build;
use uuid::Uuid;
use sloggers::types::Format;
use eventific::event::Event;
use crate::proto::service_grpc::{ExampleService, create_example_service};
use crate::proto::service::{CreateInput, CommandResult};
use std::sync::Arc;
use grpcio::{ServerBuilder, Environment, RpcContext, UnarySink};
use std::mem::{ManuallyDrop, forget};
use std::thread;

mod proto;

#[derive(Default, Debug)]
struct SimpleState {
    title: String
}

#[derive(Debug, Clone)]
enum EventData {
    Created,
    TitleChanged(String)
}

fn state_builder(mut state: SimpleState, event: &Event<EventData>) -> SimpleState {
    if let EventData::TitleChanged(title) = &event.payload {
        state.title = title.to_owned();
    }
    state
}

#[derive(Clone)]
struct GrpcService {
    eventific: Eventific<SimpleState, EventData>
}

impl GrpcService {
    fn new(eventific: Eventific<SimpleState, EventData>) -> Self {
        Self {
            eventific
        }
    }
}

impl ExampleService for GrpcService {
    fn create(&mut self, _ctx: RpcContext, req: CreateInput, sink: UnarySink<CommandResult>) {

        unimplemented!()
    }
}

/// This example showcases how you can use eventific to store and retrieve events. In a real world use case this would
/// probably not happen in the same service, you would instead have one service for persisting and another for reading
fn main() {
    let logger = TerminalLoggerBuilder::new().format(Format::Compact).build().unwrap();
    let environment = Arc::new(Environment::new(4));
    let environment_clone = Arc::clone(&environment);

    let run_future = EventificBuilder::new()
        .logger(&logger)
        .with_grpc_service(|eventific| create_example_service(GrpcService::new(eventific)))
        .start()
        .map(|_|())
        .map_err(|err| eprintln!("{}", err));

    // We always start eventific by scheduling on a executor. Tokio is one of the simplest implementations
    tokio::run(run_future);
}
