# Task 2.1: emitter 로직을 resource 모듈로 이동 및 통합

## 1. Objective (목표)

- 독립된 모듈로 존재하던 `emitter`를 `resource` 모듈의 하위 컴포넌트로 통합합니다.
- `resource` 도메인이 "데이터의 정의(Resource)"부터 "물리적 출력(Emitter)"까지를 책임지도록 구조를 단순화합니다.

## 2. Context & Files (작업 범위)

- **생성:** `src/resource/emitter.rs`
- **수정:** `src/resource/mod.rs`
- **참고:** `src/emitter/core.rs` (기존 로직)

## 3. Instructions (세부 지침)

### Step 1: 파일 이동 및 네임스페이스 조정

- `src/emitter/core.rs`의 내용을 `src/resource/emitter.rs`로 복사합니다.
- `TransformedFile`을 참조할 때 같은 모듈 내의 것을 사용하도록 수정합니다. (예: `use super::TransformedFile;`)

### Step 2: `resource/mod.rs` 업데이트

새로 추가된 `emitter` 모듈을 선언하고 외부에서 사용할 수 있도록 re-export 합니다.

```rust
// src/resource/mod.rs
pub mod emitter;
pub use emitter::Emitter;
```

### Step 3: 가시성 확인

`Emitter` 구조체와 그 메서드(`clean`, `emit`)가 `builder` 모듈에서 접근 가능한지 확인합니다.

## 4. Acceptance Criteria (검증 체크리스트)

1. `src/resource/emitter.rs` 파일이 존재하고 기존 `Emitter` 로직을 포함하는가?
2. `crate::resource::Emitter` 형태로 접근이 가능한가?
3. `emitter.rs` 내에서 `TransformedFile` 참조가 올바르게 이루어지는가?
