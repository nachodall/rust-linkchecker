use clap::Parser;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

mod models;
mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = parser::extract_title(""); //Making clippy happy

    let args = Args::parse();
    let content = fs::read_to_string(&args.input)?;
    let links = parser::extract_links(&content);

    for link in links {
        let checked_link = models::LinkCheckResult::new(link);
        println!("{}", checked_link.url);
    }

    Ok(())
}
