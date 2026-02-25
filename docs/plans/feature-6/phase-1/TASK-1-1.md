# Task 1-1: Frontmatter Extraction Utility

## 1. 개요
마크다운 파일의 텍스트에서 YAML Frontmatter 영역을 안전하게 추출하고, 본문과 분리하는 독립적인 유틸리티 기능을 구현합니다.

## 2. 상세 요구사항
- **파일 위치**: `src/utils/fs.rs`에 구현하거나 필요시 새로운 모듈 생성.
- **함수 시그니처**: `pub fn extract_frontmatter(content: &str) -> (serde_json::Value, String)`
- **로직 단계**:
    1. 입력 문자열이 `---`로 시작하는지 검사 (공백/줄바꿈 주의).
    2. 시작 구분자 이후의 첫 번째 `---` 라인을 찾음.
    3. 사이의 내용을 `serde_yaml::from_str`를 이용해 `serde_json::Value`로 변환.
    4. 파싱 실패 시 또는 Frontmatter 형식이 아닌 경우, 로그를 남기고 빈 객체 `json!({})`를 반환.
    5. 두 번째 `---` 이후의 모든 내용을 트리밍하여 반환 문자열로 구성.

## 3. 예외 상황 처리
- **Frontmatter 없음**: 전체 내용을 본문으로, 메타데이터는 빈 객체로 반환.
- **구분자가 하나만 있음**: Frontmatter가 없는 것으로 간주.
- **비어 있는 Frontmatter (`--- ---`)**: 빈 객체 반환.

## 4. 테스트 케이스 (필수 구현)
- `test_extract_standard`: 정상적인 FM과 본문 분리 확인.
- `test_extract_no_frontmatter`: FM이 없는 일반 마크다운 처리 확인.
- `test_extract_invalid_yaml`: 문법 오류가 있는 FM 처리 (Graceful failure).
