# Task 2.2: loader 모듈의 core 참조 및 테스트 수정

## 1. Objective (목표)
- `loader` 모듈에서 `src/resource`를 직접적으로 참조하던 모든 의존성을 새로운 `src/core` 모듈로 교체하고 관련 테스트를 수정합니다.

## 2. Context & Files (작업 범위)
- **수정할 파일:**
  - `src/loader/mod.rs`
  - `src/loader/filter.rs`
  - `src/loader/parser.rs`
  - `src/loader/resolver.rs`

## 3. Instructions (세부 지침)

### Step 1: `loader` 모듈 전체에서 `use` 선언 수정
- `src/loader/` 하위의 모든 파일에서 `crate::resource` 또는 `super::resource` 관련 참조를 `crate::core`로 수정합니다.

```rust
// 수정 전: src/loader/mod.rs
use crate::resource::{BuildTarget, Resource};

// 수정 후: src/loader/mod.rs
use crate::core::{BuildTarget, Resource};
```

### Step 2: `loader` 유닛 테스트 수정
- `src/loader/` 내의 모든 유닛 테스트에서 `crate::resource`를 참조하는 코드가 있다면 동일하게 `crate::core`로 수정합니다.
- 필요한 경우 테스트 환경에서 `BuildTarget` 등을 직접 생성하는 부분도 수정이 필요합니다.

## 4. Constraints (제약 사항 및 금지 행동)
- `loader` 내부의 스캔 로직이나 파싱 로직은 수정하지 마세요.

## 5. Acceptance Criteria (검증 체크리스트)
1. `src/loader/` 내의 모든 파일에서 `resource` 문자열이 사라지고 `core`로 대체되었는가? (전역적인 `resource` 참조가 아닌 경우 제외)
2. `cargo check` 실행 시 `loader` 모듈에서 컴파일 에러가 발생하지 않는가?
3. `loader` 관련 유닛 테스트가 모두 통과하는가?
