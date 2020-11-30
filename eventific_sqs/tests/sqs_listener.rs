#[macro_use]
extern crate slog;

use eventific::notification::Listener;
use eventific_sqs::SqsListener;
use futures::stream::Stream;
use rusoto_core::Region;
use rusoto_sqs::{SendMessageRequest, Sqs, SqsClient};
use slog::Logger;
use tokio::runtime::Runtime;
use uuid::Uuid;

#[test]
#[cfg_attr(not(feature = "integration_tests"), ignore)]
fn it_should_receive_message() {
    let (mut rt, listener, client) = setup_listener();

    let mut input = SendMessageRequest::default();
    input.queue_url = "http://localhost:4576/queue/eventific".to_owned();
    input.message_body = "9d2b04e9-7737-4eba-b3bc-91efac234de8".to_owned();

    rt.block_on(client.send_message(input.clone())).unwrap();
    rt.block_on(client.send_message(input.clone())).unwrap();
    rt.block_on(client.send_message(input.clone())).unwrap();

    let ids = rt.block_on(listener.listen().take(3).collect()).unwrap();

    assert_eq!(
        ids[0],
        Uuid::parse_str("9d2b04e9-7737-4eba-b3bc-91efac234de8").unwrap()
    )
}

#[test]
#[cfg_attr(not(feature = "integration_tests"), ignore)]
fn it_should_not_panic_on_invalid_uuid() {
    let (mut rt, listener, client) = setup_listener();

    let mut input = SendMessageRequest::default();
    input.queue_url = "http://localhost:4576/queue/eventific".to_owned();
    input.message_body = "INVALID_UUID".to_owned();

    rt.block_on(client.send_message(input.clone())).unwrap();
    input.message_body = "9d2b04e9-7737-4eba-b3bc-91efac234de8".to_owned();
    rt.block_on(client.send_message(input.clone())).unwrap();

    let ids = rt.block_on(listener.listen().take(1).collect()).unwrap();

    assert_eq!(
        ids[0],
        Uuid::parse_str("9d2b04e9-7737-4eba-b3bc-91efac234de8").unwrap()
    )
}

fn setup_listener() -> (Runtime, SqsListener, SqsClient) {
    let logger = Logger::root(slog::Discard, o!());
    let mut rt = Runtime::new().expect("Failed to create runtime");

    let region = Region::Custom {
        name: "us-east-1".to_string(),
        endpoint: "http://localhost:4576/queue/eventific".to_string(),
    };

    let mut listener =
        SqsListener::new_with_region("http://localhost:4576/queue/eventific", region.clone());

    rt.block_on(listener.init(&logger, "eventific")).unwrap();

    let client = SqsClient::new(region);

    (rt, listener, client)
}
