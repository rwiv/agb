# TASK-1-1: Core Constants & Code modification

`src/core/constants.rs` 및 `src/transformer/default.rs`, `src/builder/emitter.rs`에서 `OPENCODE_MD` 상수를 제거하고 `AGENTS_MD`를 사용하도록 수정합니다.

## 수정 사항

### 1. `src/core/constants.rs`
- `OPENCODE_MD` 상수 제거.
- `FORBIDDEN_FILES`에서 `OPENCODE_MD`를 `AGENTS_MD`로 대체하거나 제거(만약 이미 `AGENTS_MD`가 있다면).

### 2. `src/transformer/default.rs`
- `transform_root_prompt` 함수에서 `BuildTarget::OpenCode` 시 `AGENTS_MD`를 반환하도록 수정.

### 3. `src/builder/emitter.rs`
- `Emitter::clean`의 `targets` 배열에서 `OPENCODE_MD` 제거.

## 검증 방법
- 컴파일 에러가 없는지 확인합니다.
