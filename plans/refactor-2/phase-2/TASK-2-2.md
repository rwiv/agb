# Task 2.2: 기존 emitter 모듈 제거 및 builder 오케스트레이션 수정

## 1. Objective (목표)

- 더 이상 사용되지 않는 `src/emitter` 디렉토리를 물리적으로 제거합니다.
- `builder` 모듈이 새로운 위치(`resource::Emitter`)를 사용하도록 전체 코드를 정리합니다.

## 2. Context & Files (작업 범위)

- **수정:** `src/builder/core.rs`, `src/builder/mod.rs`, `src/main.rs`
- **삭제:** `src/emitter/` 디렉토리 전체

## 3. Instructions (세부 지침)

### Step 1: Builder 수정

`src/builder/core.rs`에서 `Emitter`를 임포트하는 경로를 수정합니다.

```rust
// 수정 전
use crate::emitter::Emitter;

// 수정 후
use crate::resource::Emitter;
```

### Step 2: 모듈 선언 제거

`src/main.rs` 또는 `lib.rs` (존재한다면)에서 `mod emitter;` 선언을 제거합니다.

### Step 3: 물리적 파일 삭제

`src/emitter/` 디렉토리와 그 하위 파일들을 삭제합니다.

## 4. Acceptance Criteria (검증 체크리스트)

1. `src/emitter` 디렉토리가 완전히 제거되었는가?
2. `builder`가 `resource::Emitter`를 사용하여 빌드 파이프라인을 정상적으로 실행하는가?
3. `cargo build` 및 `cargo check` 결과 에러가 없는가?
