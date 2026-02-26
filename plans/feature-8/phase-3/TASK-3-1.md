# TASK-3-1: 전역 임포트 경로 업데이트

## 1. 작업 개요 (Summary)
`Registry`가 `builder`에서 `core`로 이동함에 따라 `src/builder/mod.rs` 및 `src/builder/builder.rs` 등 관련 파일의 임포트 경로를 수정합니다.

## 2. 세부 구현 사항 (Implementation Details)

### 2.1. 임포트 경로 수정
- `src/builder/builder.rs`에서 `use crate::builder::registry::Registry;`를 `use crate::core::registry::Registry;`로 수정.
- 기타 관련 파일에서 `Registry` 참조 경로 수정.

### 2.2. 모듈 정의 삭제
- `src/builder/mod.rs`에서 `mod registry;` 삭제.

## 3. 검증 방법 (Verification)
- `cargo check`를 실행하여 컴파일 에러가 없는지 최종 확인.
