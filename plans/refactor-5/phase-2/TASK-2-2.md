# Task 2.2: ResourceLoader 오케스트레이션 최적화 및 최종 검증

## 1. Objective (목표)

- `ResourceLoader`가 새로 구현된 `ResourceParser`를 사용하여 리소스 로딩을 오케스트레이션하도록 최적화합니다.
- 불필요한 중복 로직을 제거하고 전체적인 모듈 응집도를 높입니다.
- 모든 통합 테스트를 성공적으로 통과시킵니다.

## 2. Context & Files (작업 범위)

- **읽기 전용 (참고용):**
  - `src/resource/loader/parser.rs` (`ResourceParser` 메서드 확인)
- **수정할 파일:**
  - `src/resource/loader/mod.rs` (`ResourceLoader` 수정)
  - `src/resource/loader/filter.rs` (필요 시 파일 검증 로직 점검)

## 3. Instructions (세부 지침)

### Step 1: `ResourceLoader` 수정 (`src/resource/loader/mod.rs`)

`ResourceLoader` 구조체에서 `MetadataParser`를 `ResourceParser`로 교체하고, `load` 메서드를 수정하여 `parser.parse_resource`를 호출하도록 합니다.

```rust
// src/resource/loader/mod.rs

pub struct ResourceLoader {
    root: PathBuf,
    filter: FileFilter,
    resolver: ResourcePathResolver,
    parser: ResourceParser, // MetadataParser에서 변경
}

impl ResourceLoader {
    pub fn load(&self) -> Result<Vec<Resource>> {
        let files = self.scan()?;
        let groups = self.resolver.resolve(&self.root, files)?;

        groups
            .into_iter()
            .map(|(key, paths)| self.parser.parse_resource(key, paths)) // 위임!
            .collect()
    }
}
```

### Step 2: `parse_resource` 중복 메서드 제거

기존 `ResourceLoader` 내부에 있던 `parse_resource` 메서드를 완전히 제거합니다. (기존 구현은 이미 `Task 2.1`에서 `ResourceParser`로 옮겨졌어야 합니다.)

### Step 3: 최종 통합 테스트

`src/resource/loader/mod.rs`의 `tests` 모듈에 있는 `test_resource_loader_load_integration` 테스트를 실행하여 정상 작동 여부를 확인합니다.

## 4. Constraints (제약 사항 및 금지 행동)

- `ResourceLoader`의 `scan` 및 `resolve` 단계에서의 로직이 손상되지 않도록 주의하세요.
- 모든 기능은 `cargo test`를 통해 검증되어야 합니다.

## 5. Acceptance Criteria (검증 체크리스트)

1. `ResourceLoader`의 코드가 획기적으로 간소화되었는가? (조립 로직이 `Parser`로 이전되었는가?)
2. `ResourceLoader::load` 통합 테스트가 성공적으로 수행되는가?
3. 전체 `agb build` 프로세스에 영향이 없는지 확인하였는가? (기존 기능 유지)
