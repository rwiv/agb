# TASK-1-2: Registry 파일 이동 및 모듈 업데이트

## 1. 작업 개요 (Summary)
`src/builder/registry.rs` 파일을 `src/core/registry.rs`로 이동하고, `src/core/mod.rs`를 업데이트하여 `registry` 모듈을 공개적으로 노출합니다.

## 2. 세부 구현 사항 (Implementation Details)

### 2.1. 파일 이동
- `src/builder/registry.rs`를 `src/core/registry.rs`로 이동.

### 2.2. 모듈 업데이트
- `src/core/mod.rs`에 `pub mod registry;` 추가.
- `src/builder/mod.rs`에서 `mod registry;` 제거 및 `use crate::core::registry::Registry;` 등 임포트 수정.

## 3. 검증 방법 (Verification)
- `cargo check`를 실행하여 파일 이동에 따른 경로 에러가 없는지 확인.
- `src/core/registry.rs` 파일이 올바르게 생성되었는지 확인.
