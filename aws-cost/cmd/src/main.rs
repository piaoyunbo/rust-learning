use structopt::StructOpt;
use dotenv::dotenv;
use infra::AWS;
use adaptor::notification::Slack;
mod usecase;
use usecase::*;

#[derive(StructOpt, Debug)]
enum Opts {
    #[structopt(name = "aws-cost", about = "check aws cost and send to slack")]
    Cost {
        #[structopt(short = "s", long = "start-date")]
        start: Option<String>,
        #[structopt(short = "e", long = "end-date")]
        end: Option<String>,
        #[structopt(short = "c", long = "channel")]
        channel: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    match Opts::from_args() {
        Opts::Cost {
            start,
            end,
            channel,
        } => {
            dotenv().ok();
            let get_cost: GetCost<AWS, Slack> = UseCase::new().await;

            get_cost.run(start, end, channel).await;
        }
    }
}