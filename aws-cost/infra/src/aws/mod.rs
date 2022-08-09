use async_trait::async_trait;
use aws_sdk_costexplorer::Client;
mod cost_explorer;

pub use cost_explorer::GetCostRequest;
pub use aws_sdk_costexplorer::output::GetCostAndUsageOutput;

pub struct AWS {
    cost_explorer_client: Client,
}

#[async_trait]
pub trait AWSRepository {
    async fn new() -> Self;
    async fn get_cost(
        &self,
        input: Option<GetCostRequest>,
    ) -> Result<GetCostAndUsageOutput, ()>;
}

#[async_trait]
impl AWSRepository for AWS {
    async fn new() -> Self {
        let shared_config = aws_config::load_from_env().await;

        AWS {
            cost_explorer_client: Client::new(&shared_config),
        }
    }

    async fn get_cost(
        &self,
        input: Option<GetCostRequest>,
    ) -> Result<GetCostAndUsageOutput, ()> {
        let res = cost_explorer::get_cost(&self.cost_explorer_client, input)
            .await
            .unwrap();

        Ok(res)
    }
}