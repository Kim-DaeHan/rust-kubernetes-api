use clap::Parser;
use k8s_openapi::api::core::v1::Pod;
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct DeploymentSpec {
    replicas: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Deployment {
    spec: DeploymentSpec,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let path: JsonPatch = serde_json::from_str(&cli.path).expect("failed");
    let deployment_name = cli.name;

    println!("{:?}", path);
    println!("{:?}", deployment_name);

    Ok(())
}
