# TASK-1-2: MetadataParser 구현 및 통합

## 목표
- JSON 및 YAML 메타데이터를 파싱하여 `serde_json::Value`로 변환하는 `MetadataParser` 객체 구현.
- 파일 확장자에 따른 분기 로직 및 에러 컨텍스트 강화를 캡슐화.

## 상세 작업 내용

### 1. `MetadataParser` 구현 (`src/resource/loader/parser.rs`)
- 인터페이스 설계:
  ```rust
  pub struct MetadataParser;

  impl MetadataParser {
      pub fn parse(&self, path: &Path) -> Result<serde_json::Value>;
  }
  ```
- 기존 `parse_resource` 내부에 있던 확장자 체크 및 역직렬화 로직 이동.
- `anyhow::Context`를 활용하여 파싱 실패 시 파일 경로와 리소스 정보가 포함되도록 개선.

### 2. `ResourceLoader`와 연동 준비
- `mod.rs`에서 `MetadataParser` 모듈 선언.

## 검증 계획
- `MetadataParser` 유닛 테스트 구현 (`src/resource/loader/parser.rs` 내):
  - 기존 `loader.rs`의 `test_parse_resource_invalid_json` 로직을 이동 및 리팩토링.
  - `.json`, `.yaml`, `.yml` 파일에 대한 정상 파싱 확인 (기존 `test_yaml_metadata_loading` 일부 로직 활용).
  - 잘못된 형식의 파일에 대해 유의미한 에러 메시지 반환 여부 확인.
  - 지원하지 않는 확장자에 대한 처리 확인.
