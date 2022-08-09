use anyhow::Result;
use chrono::Local;
use aws_sdk_costexplorer::client::Client;
use aws_sdk_costexplorer::output::GetCostAndUsageOutput;
use aws_sdk_costexplorer::model::{
    Granularity,
    GroupDefinitionType,
    date_interval,
    group_definition,
};

#[derive(Debug)]
pub struct GetCostRequest {
    pub start_date: String,
    pub end_date: String,
}

impl Default for GetCostRequest {
    fn default() -> GetCostRequest {
        let now = Local::now();

        GetCostRequest {
            start_date: now.format("%Y-%m-01").to_string(),
            end_date: now.format("%Y-%m-%d").to_string(),
        }
    }
}

pub async fn get_cost(
    client: &Client,
    input: Option<GetCostRequest>,
) -> Result<GetCostAndUsageOutput> {
    let req = input.unwrap_or_else(|| GetCostRequest::default());

    let res = client.get_cost_and_usage()
        .set_granularity(Some(Granularity::Monthly))
        .set_metrics(Some(vec!["UnblendedCost".to_string()]))
        .set_time_period(
            Some(date_interval::Builder::default()
                .set_start(Some(req.start_date))
                .set_end(Some(req.end_date))
                .build()
            )
        )
        .set_group_by(Some(vec![
            group_definition::Builder::default()
                .set_type(Some(GroupDefinitionType::Dimension))
                .set_key(Some("SERVICE".to_string()))
                .build()
        ])).send().await.unwrap();

    Ok(res)
}