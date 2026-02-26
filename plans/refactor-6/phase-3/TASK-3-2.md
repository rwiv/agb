# Task 3.2: builder 모듈의 전체 의존성 및 워크플로우 업데이트

## 1. Objective (목표)
- `builder` 모듈에서 새롭게 재구성된 `loader`, `registry`, `emitter`를 활용하여 빌드 파이프라인을 최종 연동합니다.

## 2. Context & Files (작업 범위)
- **수정할 파일:**
  - `src/builder/mod.rs`
  - `src/builder/config.rs`
  - `tests/cli_test.rs`
  - `tests/e2e_build_test.rs`

## 3. Instructions (세부 지침)

### Step 1: `builder` 모듈 내 `use` 선언 수정
- `src/builder/mod.rs` 및 하위 모듈 파일들에서 `crate::resource` 관련 참조를 `crate::core`, `crate::loader` 등으로 수정합니다.

```rust
// 수정 전: src/builder/mod.rs
use crate::resource::{ResourceLoader, Registry, Emitter, BuildTarget};

// 수정 후: src/builder/mod.rs
use crate::core::BuildTarget;
use crate::loader::ResourceLoader;
use self::registry::Registry;
use self::emitter::Emitter;
```

### Step 2: 빌드 워크플로우 연동
- `Builder::build` 함수 내에서 `ResourceLoader`를 호출하는 부분과 `Registry`, `Emitter` 인스턴스를 생성하는 부분의 경로가 올바른지 확인합니다.

### Step 3: E2E 테스트 수정
- `tests/` 디렉토리 내의 E2E 테스트들에서 `crate::resource`를 직접 참조하고 있는 코드를 새로운 모듈 구조에 맞게 수정합니다.

## 4. Constraints (제약 사항 및 금지 행동)
- `agb build`의 결과물이 이전 버전과 동일하게 생성되는지 확인해야 합니다. (비즈니스 로직 변경 금지)

## 5. Acceptance Criteria (검증 체크리스트)
1. `src/builder/` 내의 모든 파일에서 `resource` 문자열이 사라지고 `core`, `loader` 등으로 대체되었는가?
2. `cargo test` 실행 시 모든 유닛 테스트 및 E2E 테스트가 통과하는가?
3. `agb build` 실행 시 결과물 파일들이 올바르게 생성되는가?
