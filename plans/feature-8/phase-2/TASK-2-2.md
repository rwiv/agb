# TASK-2-2: E2E 및 통합 테스트 코드 수정

## 개요
테스트 코드 내에서 직접 `.json` 파일을 생성하거나 기대값으로 사용하는 로직을 수정합니다.

## 작업 내용
- `src/resource/loader/mod.rs` 내의 `tests` 수정
- `tests/e2e_build_test.rs` 내의 픽스처 생성 로직 수정
- 기타 `grep "json"`으로 검색되는 테스트 관련 코드 정리

## 검증 방법
- `cargo test` 전체 실행
