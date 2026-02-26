# Task 3-1: Validation & Integration Testing

## 1. 개요
구현된 기능이 전체 시스템에서 의도대로 작동하는지 검증하고 관련 테스트를 추가합니다.

## 2. 세부 작업
- `tests/fixtures`에 Frontmatter가 포함된 테스트용 리소스 추가.
- `src/resource/loader/parser.rs`의 유닛 테스트 보강.
- `tests/e2e_build_test.rs` 등에 메타데이터 오버라이트 시나리오 추가.
- 빌드된 결과물(예: `commands/foo.toml`)의 내용이 오버라이트된 값을 반영하는지 확인.

## 3. 검증 기준
- 모든 테스트 통과.
- `agb build` 실행 시 실제 결과 파일의 메타데이터 및 본문 구조 확인.
