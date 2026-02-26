# Task 3.1: registry.rs 및 emitter.rs를 builder 서브 모듈로 이동

## 1. Objective (목표)
- 빌드 과정에서 리소스 중복 검사와 최종 파일 출력을 담당하는 `registry`와 `emitter`를 `builder` 모듈의 서브 모듈로 이동시켜 응집도를 높입니다.

## 2. Context & Files (작업 범위)
- **이동할 파일:**
  - `src/resource/registry.rs` -> `src/builder/registry.rs`
  - `src/resource/emitter.rs` -> `src/builder/emitter.rs`
- **수정할 파일:**
  - `src/builder/mod.rs` (서브 모듈 선언 추가)

## 3. Instructions (세부 지침)

### Step 1: `src/builder` 내부로 파일 이동
- `src/resource/registry.rs`와 `src/resource/emitter.rs`를 `src/builder/` 하위로 이동합니다.
- `git mv` 명령어를 사용하는 것을 권장합니다.

```bash
git mv src/resource/registry.rs src/builder/registry.rs
git mv src/resource/emitter.rs src/builder/emitter.rs
```

### Step 2: `builder` 모듈 선언 업데이트
- `src/builder/mod.rs` 상단에 새 모듈들을 선언합니다.

```rust
// src/builder/mod.rs
pub mod registry;
pub mod emitter;
```

### Step 3: 이동된 모듈 내 `crate::resource` 참조 제거
- `src/builder/registry.rs`와 `src/builder/emitter.rs`에서 `crate::resource`를 참조하던 부분들을 `crate::core`로 수정합니다.

```rust
// 수정 전 (registry.rs)
use crate::resource::{Resource, ResourceKey};

// 수정 후 (registry.rs)
use crate::core::{Resource, ResourceKey};
```

## 4. Constraints (제약 사항 및 금지 행동)
- `Registry`와 `Emitter`의 구조체 명칭이나 기능을 변경하지 마세요.

## 5. Acceptance Criteria (검증 체크리스트)
1. `src/builder/registry.rs`와 `src/builder/emitter.rs`가 정상적으로 이동되었는가?
2. `src/builder/mod.rs`에서 두 모듈이 서브 모듈로 포함되었는가?
3. `cargo check` 시 두 모듈의 경로 관련 에러가 없는가?
