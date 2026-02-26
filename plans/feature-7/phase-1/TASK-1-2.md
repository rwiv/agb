# TASK-1-2: ResourceParser에서 JSON 파싱 로직 제거

## 개요
메타데이터 파싱 단계에서 JSON 처리 코드를 제거합니다.

## 작업 내용
- `src/resource/loader/parser.rs` 파일 수정
    - `parse_metadata` 함수의 `match` 문에서 `"json"` 암(arm)을 제거합니다.
- 관련 유닛 테스트 수정
    - `parser.rs` 내의 JSON 파싱 관련 테스트 케이스를 제거하거나 YAML로 전환합니다.

## 검증 방법
- `cargo test resource::loader::parser` 실행
