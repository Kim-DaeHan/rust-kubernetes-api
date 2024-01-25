# Rust Kubernetes Sample API

## Getting Started

1. Docker Desktop으로 Kubernetes 설치

2. kubectl version 명령어로 설치가 잘 되었는지 확인

   ```bash
    kubectl version
   ```

### Window

1. deployment 생성(해당 프로젝트 경로에서 명령어 실행)

   ```bash
    kubectl apply -f deployment.yaml
   ```

2. deployment 확인

   ```bash
    kubectl get deployments
   ```

3. cargo run(실행)

   ```bash
    cargo run -- -n=my-deployment -p="{\"op\": \"replace\", \"path\": \"/spec/replicas\", \"value\":4}"
   ```

### Mac

1. deployment 생성(해당 프로젝트 경로에서 명령어 실행)

   ```bash
    kubectl apply -f deployment.yaml
   ```

2. deployment 확인

   ```bash
    kubectl get deployments
   ```

3. cargo run(실행)

   ```bash
    cargo run -- -n=my-deployment -p='{"op": "replace", "path": "/spec/replicas", "value":4}'
   ```

### deployment의 replicas 확인하는 명령(코드가 정상 작동했는지 확인)

```bash
 kubectl get deployment my-deployment -o=jsonpath='{.spec.replicas}'
```
