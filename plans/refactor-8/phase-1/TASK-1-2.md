# 태스크 1-2: src/core/mod.rs 수정

## 1. 목표
새로 생성한 `target` 모듈을 `core` 패키지 외부에서 접근할 수 있도록 공개(export)합니다.

## 2. 작업 내용
- `src/core/mod.rs`에 `pub mod target;` 추가.
- `src/core/mod.rs`에서 `pub use target::BuildTarget;`를 추가하여 기존 경로 호환성 유지(선택 사항이나 권장됨).

## 3. 성공 기준
- `crate::core::target` 경로로 접근이 가능함.
- 프로젝트 빌드 시 모듈을 찾지 못하는 에러가 발생하지 않음.
