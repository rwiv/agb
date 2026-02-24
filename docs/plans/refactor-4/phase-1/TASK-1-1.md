# TASK-1-1: 파일 분류 로직 추출

## 1. 개요
`load_resources` 내부에 구현된 파일 그룹화 로직을 독립된 함수로 분리합니다.

## 2. 작업 내용
- `src/resource/loader.rs`에 `group_files_by_resource` 함수 정의
- 기존 86-146행의 로직을 해당 함수로 이동
- `ResourceKey` 및 `ResourcePaths` 타입 별칭 정의

```rust
type ResourceKey = (String, String, String); // (plugin, type, name)
type ResourcePaths = (Option<PathBuf>, Option<PathBuf>); // (md, metadata)

fn group_files_by_resource(
    root: &Path, 
    files: Vec<PathBuf>
) -> Result<HashMap<ResourceKey, ResourcePaths>> {
    // 기존 로직 이동
}
```

## 3. 검증 계획
- 기존 `load_resources`가 동일하게 동작하는지 확인
- 필요한 경우 `group_files_by_resource`에 대한 단위 테스트 추가
