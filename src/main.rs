use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod parser;

#[derive(Parser, Debug)]
struct Args {
    input: PathBuf,
}

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(&args.input).expect("Program could not read the input file");

    println!("File loaded successfully: {:?}", args.input);
    println!("Contenido: {} bytes", content.len());
}
