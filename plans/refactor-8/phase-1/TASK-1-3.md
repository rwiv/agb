# 태스크 1-3: 코드베이스 전체 임포트 업데이트

## 1. 목표
`BuildTarget`의 위치 변경에 따라 영향을 받는 모든 파일의 임포트 경로를 수정합니다.

## 2. 작업 내용
- `grep`을 사용하여 `BuildTarget`을 사용하는 파일 목록 식별.
- `src/main.rs`, `src/builder/config.rs`, `src/loader/parser.rs` 등 관련 파일의 임포트 문 수정.
- 필요 시 `use crate::core::BuildTarget;`에서 `use crate::core::target::BuildTarget;`으로 변경.

## 3. 성공 기준
- 모든 파일에서 `BuildTarget` 타입을 정상적으로 인식함.
- `cargo check` 실행 시 임포트 에러가 발생하지 않음.
