#[macro_use]
extern crate slog;

use eventific::notification::Sender;
use eventific_sns::SnsSender;
use rusoto_core::Region;
use slog::Logger;
use tokio::runtime::Runtime;
use uuid::Uuid;

#[test]
#[cfg_attr(not(feature = "integration_tests"), ignore)]
fn it_should_send_message() {
    let (mut rt, sender) = setup_sender("arn:aws:sns:us-east-1:123456789012:eventific");

    rt.block_on(sender.send(Uuid::parse_str("7b32824b-b8a8-4bf1-bda5-e4b0b19f50b2").unwrap()))
        .unwrap()
}

#[test]
#[cfg_attr(not(feature = "integration_tests"), ignore)]
fn it_should_fail_on_invalid_endpoint() {
    let (mut rt, sender) = setup_sender("arn:aws:sns:us-east-1:123456789012:INVALID");

    rt.block_on(sender.send(Uuid::parse_str("7b32824b-b8a8-4bf1-bda5-e4b0b19f50b2").unwrap()))
        .unwrap_err();
}

fn setup_sender(arn: &str) -> (Runtime, SnsSender) {
    let logger = Logger::root(slog::Discard, o!());
    let mut rt = Runtime::new().expect("Failed to create runtime");

    let region = Region::Custom {
        name: "us-east-1".to_string(),
        endpoint: "http://localhost:4575".to_string(),
    };

    let mut sender = SnsSender::new_with_region(arn, region.clone());

    rt.block_on(sender.init(&logger, "eventific")).unwrap();

    (rt, sender)
}
