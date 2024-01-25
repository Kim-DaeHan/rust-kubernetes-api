use clap::Parser;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{
    api::{Api, Patch, PatchParams},
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

    match op.as_str() {
        // op가 replace일 때만 replicas 수정하는 작업
        "replace" => {
            let value = cli_path.value;
            let path = cli_path.path;

            // deployment 이름
            let deployment_name = cli.name;

            // /spec/replicas 로 들어오는 path 가공
            let path_components: Vec<&str> = path.split('/').collect();
            let spec = path_components[1];
            let key = path_components[2];

            // Kubernetes 클러스터 클라이언트 생성
            let client = Client::try_default().await?;

            // Deployment에 접근하기 위해 Api 객체 생성
            let deployments: Api<Deployment> = Api::default_namespaced(client.clone());

            // fieldManager 설정(이상하게 fieldManager를 똑같이 설정해서 deployment를 생성해도 conflicts 에러 발생해서 force로 강제 수정
            let patch_params = PatchParams {
                field_manager: Some("kube".to_string()),
                force: true,
                ..Default::default()
            };

            // 변환할 내용 json객체 생성(apiVersion, kind 없으면 에러 발생)
            let patch_data = json!({
                "apiVersion": "apps/v1",
                "kind": "Deployment",
                spec: {
                    key :value
            }});

            let patch_data = Patch::Apply(&patch_data);

            // Deployment 수정
            deployments
                .patch(&deployment_name, &patch_params, &patch_data)
                .await?;
        }
        // op가 다른 연산일 때 (ex add) 필요한 로직 작성
        _ => {
            println!("replace 말고 다른 연산 로직 추가된다면 실행");
        }
    }

    Ok(())
}
