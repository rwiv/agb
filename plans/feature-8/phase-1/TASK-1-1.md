# TASK-1-1: ResourcePathResolver에서 JSON 확장자 제거

## 개요
메타데이터 탐색 단계에서 JSON 파일을 무시하도록 수정합니다.

## 작업 내용
- `src/resource/loader/resolver.rs` 파일 수정
    - `is_metadata_extension` 함수에서 `"json"`을 제거합니다.
    - `validate_metadata_uniqueness` 로직이 YAML/YML 충돌을 정상적으로 잡아내는지 확인합니다.
- 관련 유닛 테스트 수정
    - `resolver.rs` 내의 테스트 케이스 중 `.json`을 사용하는 부분을 수정하거나 제거합니다.

## 검증 방법
- `cargo test resource::loader::resolver` 실행
