# TASK-1-1: Transformer 인터페이스 개선 및 경로 계산 최적화

## 목적
타겟 파일 경로를 얻기 위해 매번 `transform`을 수행하는 비효율을 제거하고, 동기화 과정에서 타겟 경로를 명확히 식별할 수 있는 수단을 마련합니다.

## 작업 내용

1. **Transformer 트레이트 수정 (`src/transformer/mod.rs`)**
   - 리소스 타입과 이름을 인자로 받아 예상되는 타겟 경로를 반환하는 `get_target_path(&self, r_type: ResourceType, name: &str) -> PathBuf` 메서드 추가.

2. **구현체 반영**
   - `GeminiTransformer` (`src/transformer/gemini.rs`): Commands는 `.toml`, 나머지는 `.md` 경로 반환 로직 구현.
   - `DefaultTransformer` (`src/transformer/default.rs`): 모든 리소스에 대해 `.md` 경로 반환 로직 구현.

3. **Syncer 코드 수정 (`src/syncer/mod.rs`)**
   - `sync_resource` 함수 내에서 `transformer.transform(resource)` 호출 대신 `transformer.get_target_path(...)`를 사용하도록 변경.

## 검증 방법
- `cargo test`를 통해 기존 빌드 로직이 깨지지 않았는지 확인.
- `Syncer`가 올바른 타겟 경로를 참조하여 파일을 읽어오는지 디버그 로그로 확인.
