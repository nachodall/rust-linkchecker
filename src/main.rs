use clap::Parser;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use tokio::task::JoinSet;

use crate::models::LinkCheckerError;

mod client;
mod models;
mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    input: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), LinkCheckerError> {
    let args = Args::parse();
    let content = fs::read_to_string(&args.input)
        .map_err(|e| models::LinkCheckerError::IoError(e.to_string()))?;

    let links = parser::extract_links(&content);
    let http_client = reqwest::Client::new();

    let mut set = JoinSet::new();

    for link in links {
        let client = http_client.clone();
        set.spawn(async move {
            let mut res = models::LinkCheckResult::new(link);
            client::check_url(&client, &mut res).await;
            res
        });
    }

    let mut successful_checks = 0;
    let mut total_checks = 0;

    while let Some(res) = set.join_next().await {
        let check_result = res?;
        if check_result.is_ok() {
            successful_checks += 1;
        }
        total_checks += 1;
        println!("{}", check_result.produce_link_checker_report());
    }

    println!(
        "\n> [Summary] {} links worked out of {} total links checked.",
        successful_checks, total_checks
    );
    Ok(())
}
