use eventific::notification::{Listener, NotificationError};
use slog::Logger;
use futures::{Stream, Future, IntoFuture};
use uuid::Uuid;
use rusoto_core::{Region, RusotoError};
use std::str::FromStr;
use rusoto_sqs::{SqsClient, Sqs, ReceiveMessageRequest, DeleteMessageRequest, ReceiveMessageResult, ReceiveMessageError, Message};
use futures::stream::repeat;
use futures::stream::iter_ok;
use futures::future::Either;
use tokio::timer::Delay;
use std::time::{Instant, Duration};
use std::ops::Add;

pub struct SqsListener {
    queue_url: String,
    region: Region,
    logger: Option<Logger>
}

impl SqsListener {

    pub fn new(queue_url: &str, region: &str) -> Self {
        Self::new_with_region(queue_url, Region::from_str(region).expect("Not a valid region"))
    }

    pub fn new_with_region(queue_url: &str, region: Region) -> Self {
        Self {
            queue_url: queue_url.to_owned(),
            region,
            logger: None
        }
    }

    fn handle_message(logger: Logger, result: Result<ReceiveMessageResult, RusotoError<ReceiveMessageError>>, client: SqsClient, queue_url: String) -> impl Future<Item=Vec<Uuid>, Error=NotificationError> {
        match result {
            Ok(message) => Either::A(Self::handle_message_result(&logger, message, client, queue_url)),
            Err(err) => Either::B(Self::handle_message_error(&logger, err)),
        }
    }

    fn handle_message_result(logger: &Logger, result: ReceiveMessageResult, client: SqsClient, queue_url: String) -> impl Future<Item=Vec<Uuid>, Error=NotificationError> {
        let messages = result.messages.unwrap_or_default();
        let ids: Vec<_> = messages
            .into_iter()
            .map(|mess| Self::map_message(&logger, mess, &client, queue_url.to_owned()))
            .filter(|i| i != &Uuid::default())
            .collect();

        futures::finished(ids)
    }

    fn map_message(logger: &Logger, message: Message, client: &SqsClient, queue_url: String) -> Uuid {
        Self::delete_message(&logger, message.receipt_handle.clone(), &client, queue_url);
        Self::parse_uuid(&logger, message.body)
    }

    fn parse_uuid(logger: &Logger, body: Option<String>) -> Uuid {
        match body {
            Some(u) => {
                match Uuid::parse_str(&u) {
                    Ok(uuid) => {
                        uuid
                    },
                    Err(_) => {
                        warn!(logger, "Received an invalid uuid, the invalid id was: {}", u);
                        Uuid::default()
                    },
                }
            },
            None => {
                warn!(logger, "The SQS Message didn't contain a body, strange...");
                Uuid::default()
            },
        }
    }

    fn delete_message(logger: &Logger, handle: Option<String>, client: &SqsClient, queue_url: String) {
        if let Some(receipt_handle) = handle {
            let err_log = logger.clone();
            let delete_fut  = client.delete_message(DeleteMessageRequest {
                queue_url: queue_url.to_owned(),
                receipt_handle
            })
                .map_err(|_err| ())
                .map(move |_| warn!(err_log, "Failed to delete SQS Message"));
            tokio::spawn(delete_fut);
        }
    }

    fn handle_message_error(logger: &Logger, error: RusotoError<ReceiveMessageError>) -> impl Future<Item=Vec<Uuid>, Error=NotificationError> {
        match error {
            RusotoError::HttpDispatch(err) => {
                warn!(logger, "Network error"; "error" => format!("{}", err));
                Either::A(
                    Delay::new(Instant::now().add(Duration::from_millis(1000)))
                        .map_err(|d_err| NotificationError::FailedToListen(format_err!("{}", d_err)))
                        .and_then(|_| Ok(vec![]))
                )
            },
            _ => Either::B(futures::failed(NotificationError::FailedToListen(format_err!("{}", error))))
        }
    }
}

impl Listener for SqsListener {
    fn init(&mut self, logger: &Logger, service_name: &str) -> Box<dyn Future<Item=(), Error=NotificationError> + Send> {
        let log = logger.new(o!("service" => service_name.to_owned(), "module" => "SqsListener"));
        info!(log, "Initializing new SQS Listener");
        self.logger.replace(log);

        Box::new(futures::finished(()))
    }

    fn listen(&self) -> Box<dyn Stream<Item=Uuid, Error=NotificationError> + Send> {
        let client = SqsClient::new(self.region.clone());
        let mut input = ReceiveMessageRequest::default();
        input.queue_url = self.queue_url.to_owned();
        let logger = self.logger.clone().expect("The listener needs to be initialized");
        let queue_url = self.queue_url.clone().to_owned();

        info!(logger, "Starting to listen on notifications");

        let stream = repeat(()).and_then(move |_| {
            let other_client = client.clone();
            let other_queue_url = queue_url.clone();
            let log = logger.clone();

            client.receive_message(input.clone())
                .then(move |res| Self::handle_message(log, res, other_client, other_queue_url))
        })
            .map(|ids| iter_ok(ids))
            .flatten();

        Box::new(stream)
    }
}
