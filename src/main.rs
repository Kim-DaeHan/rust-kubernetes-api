use clap::Parser;
use kube::{
    api::{Api, PatchParams, PatchStrategy, Resource},
    config,
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct JsonPatch {
    op: String,
    path: String,
    value: u32,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    path: String,
}

fn main() {
    println!("Hello, world!");

    let cli = Cli::parse();
    let path: JsonPatch = serde_json::from_str(&cli.path).expect("failed");

    println!("{:?}", path)
}
