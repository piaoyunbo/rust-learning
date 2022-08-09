use async_trait::async_trait;
use infra::{
    AWSRepository,
    GetCostAndUsageOutput,
    GetCostRequest,
    deserialize,
};
use adaptor::notification::NotificationModule;

#[async_trait]
pub trait UseCase {
    async fn new() -> Self;
    async fn run(&self, start: Option<String>, end: Option<String>, channel: Option<String>);
}

pub struct GetCost<T, U>
    where
        T: AWSRepository,
        U: NotificationModule,
{
    aws_repository: T,
    slack: U,
}

#[async_trait]
impl<T: Sync + Send + AWSRepository, U: Sync + Send + NotificationModule> UseCase for GetCost<T, U> {
    async fn new() -> Self {
        let aws_repository = AWSRepository::new().await;
        let slack = NotificationModule::new();

        GetCost {
            aws_repository,
            slack,
        }
    }

    async fn run(&self, start: Option<String>, end: Option<String>, channel: Option<String>) {
        let cost: GetCostAndUsageOutput;
        let channel = channel.unwrap_or_else(|| "#cost".to_string());

        if start.is_none() || end.is_none() {
            cost = self.aws_repository.get_cost(None).await.unwrap();
        } else {
            let req = GetCostRequest {
                start_date: start.unwrap(),
                end_date: end.unwrap(),
            };

            cost = self.aws_repository.get_cost(Some(req)).await.unwrap();
        }

        let body_list = deserialize(cost, channel.clone()).unwrap();

        for body in body_list {
            self.slack.send(body).await;
        }
    }
}