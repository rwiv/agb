# Task 1.1: src/core 모듈 생성 및 모델 이동

## 1. Objective (목표)
- 프로젝트 전역에서 사용되는 공용 데이터 모델들을 `src/core` 모듈로 통합하여 의존성 구조의 최하단 레이어를 구축합니다.

## 2. Context & Files (작업 범위)
- **이동할 파일:** `src/resource/model.rs` -> `src/core/model.rs`
- **생성할 파일:** `src/core/mod.rs`
- **수정할 파일:** `src/lib.rs` 또는 `src/main.rs` (모듈 선언)

## 3. Instructions (세부 지침)

### Step 1: `src/core` 디렉토리 및 파일 생성
- `src/core/mod.rs`를 생성하고, `model.rs`를 서브 모듈로 선언한 뒤 내부 타입을 public으로 export합니다.

```rust
// src/core/mod.rs
pub mod model;

pub use model::*;
```

### Step 2: 모델 파일 이동
- `src/resource/model.rs`의 내용을 `src/core/model.rs`로 복사합니다.
- 복사된 파일 상단에서 `anyhow` 등 필요한 의존성이 올바른지 확인합니다 (기존 코드 유지).

### Step 3: 최상위 모듈 선언 추가
- `src/main.rs` (또는 프로젝트 구조에 따라 `lib.rs`)에 `pub mod core;`를 추가하여 모듈을 등록합니다.

## 4. Constraints (제약 사항 및 금지 행동)
- `Resource`, `BuildTarget`, `TransformedFile` 등의 구조체 이름을 변경하지 마세요.
- 기존 `src/resource/model.rs`는 모든 작업이 끝날 때까지 삭제하지 말고, 일단 `core`로 복제 후 진행하는 것을 권장합니다 (안전한 이주를 위해).

## 5. Acceptance Criteria (검증 체크리스트)
1. `src/core/mod.rs`와 `src/core/model.rs`가 정상적으로 생성되었는가?
2. `crate::core::Resource`와 같은 방식으로 다른 모듈에서 참조가 가능한가?
3. `cargo check` 실행 시 `core` 모듈에 대한 타입 에러가 없는가?
