# Task 1.2: transformer 모듈의 의존성 업데이트

## 1. Objective (목표)
- `transformer` 모듈에서 `src/resource`를 직접적으로 참조하던 모든 의존성을 새로운 `src/core` 모듈로 교체합니다.

## 2. Context & Files (작업 범위)
- **수정할 파일:**
  - `src/transformer/mod.rs`
  - `src/transformer/default.rs`
  - `src/transformer/gemini.rs`

## 3. Instructions (세부 지침)

### Step 1: `transformer` 모듈 전체에서 `use` 선언 수정
- `src/transformer/mod.rs` 및 하위 모듈 파일들에서 `crate::resource` 또는 `super::resource` 관련 참조를 `crate::core`로 수정합니다.

```rust
// 수정 전: src/transformer/mod.rs
use crate::resource::{BuildTarget, Resource, TransformedFile};

// 수정 후: src/transformer/mod.rs
use crate::core::{BuildTarget, Resource, TransformedFile};
```

### Step 2: `transformer` 유닛 테스트 수정
- `src/transformer/mod.rs` 하단의 `tests` 모듈 내에서 `crate::resource`를 참조하는 코드가 있다면 동일하게 `crate::core`로 수정합니다.

## 4. Constraints (제약 사항 및 금지 행동)
- `transformer` 내부의 비즈니스 로직(변환 로직)은 수정하지 마세요. 오직 의존성 경로만 수정합니다.

## 5. Acceptance Criteria (검증 체크리스트)
1. `src/transformer/` 내의 모든 파일에서 `resource` 문자열이 사라지고 `core`로 대체되었는가? (전역적인 `resource` 참조가 아닌 경우 제외)
2. `cargo check` 실행 시 `transformer` 모듈에서 컴파일 에러가 발생하지 않는가?
3. `transformer` 관련 유닛 테스트가 모두 통과하는가?
