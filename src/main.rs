use clap::Parser;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use tokio::task::JoinSet;

mod client;
mod models;
mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    input: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = parser::extract_title("");

    let args = Args::parse();
    let content = fs::read_to_string(&args.input)?;
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

    while let Some(res) = set.join_next().await {
        let res = res?;
        println!("{}", res.produce_link_checker_report());
    }

    Ok(())
}
