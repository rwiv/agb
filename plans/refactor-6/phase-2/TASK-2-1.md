# Task 2.1: src/resource/loader를 src/loader로 이동

## 1. Objective (목표)
- `src/resource/loader` 모듈을 프로젝트 최상위 모듈인 `src/loader`로 승격시킵니다.

## 2. Context & Files (작업 범위)
- **이동할 디렉토리:** `src/resource/loader/` -> `src/loader/`
- **수정할 파일:**
  - `src/main.rs` (또는 `lib.rs`) (모듈 선언 추가)
  - `src/loader/mod.rs` (내부 서브 모듈 선언 확인)

## 3. Instructions (세부 지침)

### Step 1: `src/loader` 디렉토리로 파일 이동
- `src/resource/loader/` 디렉토리 전체를 `src/loader/`로 이동합니다.
- `git mv` 명령어를 사용하여 이력 관리를 유지하는 것을 권장합니다.

```bash
git mv src/resource/loader src/loader
```

### Step 2: 최상위 모듈 선언 수정
- `src/main.rs` (또는 `lib.rs`)에서 `pub mod loader;`를 추가하여 모듈을 활성화합니다.

### Step 3: `src/loader` 내부의 서브 모듈 구조 확인
- `src/loader/mod.rs`에서 `filter`, `parser`, `resolver` 모듈 선언이 올바른지 확인합니다.

```rust
// src/loader/mod.rs
pub mod filter;
pub mod parser;
pub mod resolver;

pub use mod::ResourceLoader; // 필요한 경우
```

## 4. Constraints (제약 사항 및 금지 행동)
- `loader` 디렉토리 내부의 파일명(`filter.rs`, `parser.rs`, `resolver.rs`)을 변경하지 마세요.

## 5. Acceptance Criteria (검증 체크리스트)
1. `src/loader/` 디렉토리가 최상위에 존재하고 하위 파일들이 모두 포함되어 있는가?
2. `src/main.rs`에서 `loader` 모듈이 정상적으로 선언되어 있는가?
3. `cargo check` 시 로더 모듈의 경로 관련 에러가 없는가?
