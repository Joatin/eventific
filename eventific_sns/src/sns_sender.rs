use eventific::notification::NotificationError;
use eventific::notification::Sender;
use futures::Future;
use rusoto_core::Region;
use rusoto_sns::{PublishInput, Sns, SnsClient};
use slog::Logger;
use std::str::FromStr;
use uuid::Uuid;

pub struct SnsSender {
    region: Region,
    topic_arn: String,
    logger: Option<Logger>,
    client: Option<SnsClient>,
}

impl SnsSender {
    pub fn new(topic_arn: &str, region: &str) -> Self {
        Self::new_with_region(
            topic_arn,
            Region::from_str(region).expect("Not a valid region"),
        )
    }

    pub fn new_with_region(topic_arn: &str, region: Region) -> Self {
        Self {
            region,
            topic_arn: topic_arn.to_owned(),
            logger: None,
            client: None,
        }
    }
}

impl Sender for SnsSender {
    fn init(
        &mut self,
        logger: &Logger,
        service_name: &str,
    ) -> Box<dyn Future<Item = (), Error = NotificationError> + Send> {
        let log = logger.new(o!("service" => service_name.to_owned(), "module" => "SnsSender"));

        info!(log, "Initializing new SNS Sender, yay");

        self.logger.replace(log);

        let client = SnsClient::new(self.region.clone());
        self.client.replace(client);

        Box::new(futures::finished(()))
    }

    fn send(
        &self,
        aggregate_id: Uuid,
    ) -> Box<dyn Future<Item = (), Error = NotificationError> + Send> {
        let client = self.client.clone().expect("Sender must be initialized");

        let mut input = PublishInput::default();
        input.topic_arn = Some(self.topic_arn.to_owned());
        input.message = aggregate_id.to_string();

        let res = client
            .publish(input)
            .map(|_| ())
            .map_err(|err| NotificationError::FailedToSend(format_err!("{}", err)));

        Box::new(res)
    }
}
