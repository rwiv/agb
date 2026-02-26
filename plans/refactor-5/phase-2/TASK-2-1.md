# Task 2.1: MetadataParser를 ResourceParser로 리네이밍 및 기능 확장

## 1. Objective (목표)

- `MetadataParser`를 `ResourceParser`로 리네이밍하여 단순히 메타데이터만 파싱하는 것이 아니라, 전체 리소스를 조립하는 역할을 부여합니다.
- `ResourceLoader`에 있던 `parse_resource` 로직을 `ResourceParser` 내부로 이동시킵니다.
- 이를 통해 리소스 조립 로직을 독립적으로 테스트할 수 있는 환경을 구축합니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `src/resource/loader/resolver.rs` (`ResourceKey`, `ResourcePaths` 참조)
- **수정할 파일:**
  - `src/resource/loader/parser.rs` (`MetadataParser` -> `ResourceParser` 리네이밍 및 로직 추가)
  - `src/resource/loader/mod.rs` (`ResourceParser` 참조 및 `parse_resource` 제거 시작)

## 3. Instructions (세부 지침)

### Step 1: `MetadataParser` 리네이밍 및 필드 추가 (`src/resource/loader/parser.rs`)

`MetadataParser` 구조체를 `ResourceParser`로 이름을 바꾸고, `mod.rs`의 내용을 바탕으로 `parse_resource` 메서드를 구현합니다.

```rust
// src/resource/loader/parser.rs

pub struct ResourceParser;

impl ResourceParser {
    pub fn new() -> Self {
        Self
    }

    /// (기존 parse 로직 유지...)
    pub fn parse_metadata(&self, path: &Path, resource_type: &str, resource_name: &str) -> Result<Value> { ... }

    /// 그룹화된 파일 경로들로부터 Resource 객체를 생성합니다.
    pub fn parse_resource(&self, key: ResourceKey, paths: ResourcePaths) -> Result<Resource> {
        // ResourceLoader에서 가져온 로직 구현
        // fs::read_to_string(paths.md) 등을 여기서 수행
    }
}
```

### Step 2: 공통 타입 이동 (`ResourceKey`, `ResourcePaths`)

`ResourceKey`와 `ResourcePaths`는 이제 `Resolver`와 `Parser` 양쪽에서 사용되므로, 이들을 `src/resource/loader/resolver.rs`에서 `src/resource/model.rs`로 이동시켜 공통 데이터 모델로 관리합니다.

- `src/resource/model.rs`에 두 구조체 정의 추가. (기존 `resolver.rs`에서 정의를 가져옵니다.)
- `src/resource/loader/resolver.rs` 및 `src/resource/loader/parser.rs`에서 `crate::resource::model`을 통해 참조하도록 수정합니다.
- `src/resource/mod.rs`에서 `pub use model::{ResourceKey, ResourcePaths};`를 추가하여 편리하게 접근할 수 있도록 합니다.

### Step 3: `ResourceParser` 테스트 강화

`parse_resource`가 독립적으로 잘 작동하는지 확인하는 단위 테스트를 `parser.rs`에 추가합니다.

## 4. Constraints (제약 사항 및 금지 행동)

- 기존의 메타데이터 파싱 로직(JSON/YAML)이 깨지지 않도록 주의하세요.
- `ResourceLoader`와의 연동은 `Task 2.2`에서 완료하되, 여기서는 `ResourceParser`의 독립적 완결성에 집중하세요.

## 5. Acceptance Criteria (검증 체크리스트)

1. `ResourceParser::parse_resource` 메서드가 정상적으로 구현되었는가?
2. 마크다운 본문이 없는 경우와 메타데이터가 없는 경우 모두 안전하게 처리되는가?
3. `parser.rs`의 단위 테스트가 모두 통과하는가?
