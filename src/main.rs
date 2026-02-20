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
    let _ = parser::extract_title(""); //Making clippy happy

    let args = Args::parse();
    let content = fs::read_to_string(&args.input)
        .map_err(|e| models::LinkCheckerError::IoError(e.to_string()))?;

    let links = parser::extract_links(&content);
    let http_client = reqwest::Client::new();

    for link in links {
        let mut res = models::LinkCheckResult::new(link);
        client::check_url(&http_client, &mut res).await;
        println!("{}", res.produce_link_checker_report());
    }

    println!(
        "\n> [Summary] {} links worked out of {} total links checked.",
        successful_checks, total_checks
    );
    Ok(())
}
