use clap::Parser;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams, Patch, PatchParams, ResourceExt},
    Client,
};
use serde::Deserialize;
use serde_json::json;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let cli_path: JsonPatch = serde_json::from_str(&cli.path).expect("failed");
    let op = cli_path.op;
    let value = cli_path.value;
    let path = cli_path.path;

    let deployment_name = cli.name;

    let client = Client::try_default().await?;

    // Deployment에 접근하기 위해 Api 객체 생성
    let deployments: Api<Deployment> = Api::default_namespaced(client.clone());
    // 생성한 Deployment들 확인 하는 코드
    for d in deployments.list(&ListParams::default()).await? {
        println!("found deployment {}", d.name_any());
    }

    // Deployment 이름으로부터 Deployment 리소스 가져오기
    let deployment = deployments.get(&deployment_name).await?;

    let patch_params = PatchParams {
        field_manager: Some("my-deployment".to_string()),
        ..Default::default()
    };
    let patch_data = json!({
        "op": op,
        "path": path,
        "value": value,
    });
    let patch_data = Patch::Apply(&patch_data);

    // Deployment 수정
    deployments
        .patch(&deployment_name, &patch_params, &patch_data)
        .await?;

    // 수정된 Deployment 확인 또는 다른 작업 수행
    let modified_deployment = deployments.get(&deployment_name).await?;
    println!("Modified Deployment: {:?}", modified_deployment);

    // 생성한 pod 확인 하는 코드
    let pods: Api<Pod> = Api::default_namespaced(client);

    for p in pods.list(&ListParams::default()).await? {
        println!("found pod {}", p.name_any());
    }

    Ok(())
}
