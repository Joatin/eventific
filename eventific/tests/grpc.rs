extern crate eventific;

mod proto;

use eventific::EventificBuilder;
use eventific::Eventific;
use eventific::store::MemoryStore;
use futures::future::Future;
use crate::proto::service_grpc::{ExampleService, ExampleServiceServer};
use crate::proto::service_grpc::ExampleServiceClient;
use crate::proto::service::{CreateInput, ChangeTitleInput};
use crate::proto::service::CommandResult;
use std::borrow::ToOwned;
use grpc::{RequestOptions, SingleResponse};
use grpc::ClientStubExt;
use grpc::GrpcStatus;
use std::thread;
use failure::_core::time::Duration;


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

#[cfg(feature = "with_grpc")]
impl ExampleService for GrpcService {
    fn create(&self, o: RequestOptions, p: CreateInput) -> SingleResponse<CommandResult> {
        self.eventific.grpc_create_aggregate(
            o,
            p,
            |r| &r.aggregateId,
            |_| {
                Ok(vec![
                    EventData::Created
                ])
            },
            create_result
        )
    }

    fn change_title(&self, o: RequestOptions, p: ChangeTitleInput) -> SingleResponse<CommandResult> {
        self.eventific.grpc_add_events_to_aggregate(
            o,
            p,
            |r| &r.aggregateId,
            |_, _| {
                Ok(vec![
                    EventData::TitleChanged("Hello World!".to_owned())
                ])
            },
            create_result
        )
    }
}

#[cfg(feature = "with_grpc")]
#[test]
fn it_should_store_events() {
    let port = 10003;
    let mut rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let start_future = EventificBuilder::new()
        .grpc_port(port)
        .with_grpc_service(|eventific| {
            ExampleServiceServer::new_service_def(GrpcService::new(eventific))
        })
        .start()
        .map(|_|())
        .map_err(|err| eprintln!("{}", err));

    rt.spawn(start_future);

    thread::sleep(Duration::from_millis(500));

    let client = ExampleServiceClient::new_plain("::1", port, Default::default()).unwrap();

    let mut input = CreateInput::default();
    input.set_aggregateId("1e629a2c-2d92-46b1-897a-dc429e789d6b".to_owned());
    client.create(Default::default(), input.clone()).wait().unwrap();
    input.set_aggregateId("2e629a2c-2d92-46b1-897a-dc429e789d6b".to_owned());
    client.create(Default::default(), input.clone()).wait().unwrap();
    input.set_aggregateId("3e629a2c-2d92-46b1-897a-dc429e789d6b".to_owned());
    client.create(Default::default(), input.clone()).wait().unwrap();

    rt.shutdown_now().wait().unwrap();
}

#[cfg(feature = "with_grpc")]
#[test]
fn it_should_return_already_exists_if_the_aggregate_already_exists() {
    let port = 10002;
    let mut rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let start_future = EventificBuilder::new()
        .grpc_port(port)
        .with_grpc_service(|eventific| {
            ExampleServiceServer::new_service_def(GrpcService::new(eventific))
        })
        .start()
        .map(|_|())
        .map_err(|err| eprintln!("{}", err));

    rt.spawn(start_future);

    thread::sleep(Duration::from_millis(500));

    let client = ExampleServiceClient::new_plain("::1", port, Default::default()).unwrap();

    let mut input = CreateInput::default();
    input.set_aggregateId("1e629a2c-2d92-46b1-897a-dc429e789d6a".to_owned());
    client.create(Default::default(), input.clone()).wait().unwrap();
    input.set_aggregateId("1e629a2c-2d92-46b1-897a-dc429e789d6a".to_owned());

    let err: grpc::Error = client.create(Default::default(), input.clone()).wait().unwrap_err();

    if let grpc::Error::GrpcMessage(message_err) = err {
        if message_err.grpc_status != GrpcStatus::AlreadyExists as _ {
            panic!("Not correct error code");
        }
    } else {
        panic!("Wrong error response");
    }
    rt.shutdown_now().wait().unwrap();
}

#[cfg(feature = "with_grpc")]
#[test]
fn it_should_add_events_to_aggregate() {
    let port = 10001;
    let mut rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let start_future = EventificBuilder::new()
        .grpc_port(port)
        .with_grpc_service(|eventific| {
            ExampleServiceServer::new_service_def(GrpcService::new(eventific))
        })
        .start()
        .map(|_|())
        .map_err(|err| eprintln!("{}", err));

    rt.spawn(start_future);

    thread::sleep(Duration::from_millis(500));

    let client = ExampleServiceClient::new_plain("::1", port, Default::default()).unwrap();

    let mut create_input = CreateInput::default();
    create_input.set_aggregateId("1a629a2c-2d92-46b1-897a-dc429e789d6a".to_owned());
    client.create(Default::default(), create_input).wait().unwrap();

    let mut title_input = ChangeTitleInput::default();
    title_input.set_aggregateId("1a629a2c-2d92-46b1-897a-dc429e789d6a".to_owned());
    title_input.set_title("Hello World".to_owned());
    client.change_title(Default::default(), title_input).wait().unwrap();
    rt.shutdown_now().wait().unwrap();
}
