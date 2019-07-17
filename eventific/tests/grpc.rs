extern crate eventific;

mod proto;

use eventific::EventificBuilder;
use eventific::Eventific;
use eventific::store::MemoryStore;
use futures::future::Future;
use crate::proto::service_grpc::ExampleService;
use crate::proto::service_grpc::ExampleServiceClient;
use crate::proto::service_grpc::create_example_service;
use crate::proto::service::{CreateInput, ChangeTitleInput};
use grpcio::RpcContext;
use grpcio::UnarySink;
use crate::proto::service::CommandResult;
use sloggers::terminal::TerminalLoggerBuilder;
use sloggers::Build;
use sloggers::types::Format;
use grpcio::ChannelBuilder;
use std::sync::Arc;
use std::borrow::ToOwned;
use grpcio::RpcStatusCode;


#[derive(Default, Debug)]
struct SimpleState {
    title: String
}

#[derive(Debug, Clone)]
enum EventData {
    Created,
    TitleChanged(String)
}

#[derive(Clone)]
struct GrpcService {
    eventific: Eventific<SimpleState, EventData, MemoryStore<EventData>>
}

impl GrpcService {
    fn new(eventific: Eventific<SimpleState, EventData, MemoryStore<EventData>>) -> Self {
        Self {
            eventific
        }
    }
}

fn create_result() -> CommandResult {
    let mut res = CommandResult::default();
    res.set_result("OK".to_owned());
    res
}

#[cfg(feature = "grpc")]
impl ExampleService for GrpcService {
    fn create(&mut self, ctx: RpcContext, req: CreateInput, sink: UnarySink<CommandResult>) {
        self.eventific.grpc_create_aggregate(
            ctx,
            req,
            sink,
            |r| &r.aggregateId,
            |_| {
                Ok(vec![
                    EventData::Created
                ])
            },
            create_result
        );
    }

    fn change_title(&mut self, ctx: RpcContext, req: ChangeTitleInput, sink: UnarySink<CommandResult>) {
        self.eventific.grpc_add_events_to_aggregate(
            ctx,
            req,
            sink,
            |r| &r.aggregateId,
            |req, _aggregate| {
                Ok(vec![
                    EventData::TitleChanged(req.get_title().to_owned())
                ])
            },
            create_result
        );
    }
}

#[cfg(feature = "grpc")]
#[test]
fn it_should_store_events() {
    let mut rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let start_future = EventificBuilder::new()
        .with_grpc_service(|eventific| {
            create_example_service(GrpcService::new(eventific))
        })
        .start()
        .map(|_|())
        .map_err(|err| eprintln!("{}", err));

    rt.block_on(start_future);

    let channel = ChannelBuilder::new(Arc::new(grpcio::Environment::new(4)))
        .connect("localhost:5000");
    let client = ExampleServiceClient::new(channel);

    let mut input = CreateInput::default();
    input.set_aggregateId("1e629a2c-2d92-46b1-897a-dc429e789d6b".to_owned());
    client.create(&input).unwrap();
    input.set_aggregateId("2e629a2c-2d92-46b1-897a-dc429e789d6b".to_owned());
    client.create(&input).unwrap();
    input.set_aggregateId("3e629a2c-2d92-46b1-897a-dc429e789d6b".to_owned());
    client.create(&input).unwrap();

    rt.shutdown_now().wait();
}

#[cfg(feature = "grpc")]
#[test]
fn it_should_return_already_exists_if_the_aggregate_already_exists() {
    let mut rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let start_future = EventificBuilder::new()
        .with_grpc_service(|eventific| {
            create_example_service(GrpcService::new(eventific))
        })
        .start()
        .map(|_|())
        .map_err(|err| eprintln!("{}", err));

    rt.block_on(start_future);

    let channel = ChannelBuilder::new(Arc::new(grpcio::Environment::new(4)))
        .connect("localhost:5000");
    let client = ExampleServiceClient::new(channel);

    let mut input = CreateInput::default();
    input.set_aggregateId("1e629a2c-2d92-46b1-897a-dc429e789d6a".to_owned());
    client.create(&input).unwrap();
    input.set_aggregateId("1e629a2c-2d92-46b1-897a-dc429e789d6a".to_owned());
    let err: grpcio::Error = client.create(&input).unwrap_err();
    if let grpcio::Error::RpcFailure(status) = err {
        if status.status != RpcStatusCode::AlreadyExists {
            panic!("Not correct error code");
        }
    } else {
        panic!("Wrong error response");
    }
    rt.shutdown_now().wait();
}

#[cfg(feature = "grpc")]
#[test]
fn it_should_add_events_to_aggregate() {
    let mut rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let start_future = EventificBuilder::new()
        .with_grpc_service(|eventific| {
            create_example_service(GrpcService::new(eventific))
        })
        .start()
        .map(|_|())
        .map_err(|err| eprintln!("{}", err));

    rt.block_on(start_future);

    let channel = ChannelBuilder::new(Arc::new(grpcio::Environment::new(4)))
        .connect("localhost:5000");
    let client = ExampleServiceClient::new(channel);

    let mut create_input = CreateInput::default();
    create_input.set_aggregateId("1a629a2c-2d92-46b1-897a-dc429e789d6a".to_owned());
    client.create(&create_input).unwrap();

    let mut title_input = ChangeTitleInput::default();
    title_input.set_aggregateId("1a629a2c-2d92-46b1-897a-dc429e789d6a".to_owned());
    title_input.set_title("Hello World".to_owned());
    client.change_title(&title_input).unwrap();
    rt.shutdown_now().wait();
}
