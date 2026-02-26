# TASK 2-2: Implement `detransform` for `GeminiTransformer`

## 개요 (Description)
`GeminiTransformer`의 `detransform` 로직을 구현하여 Gemini 전용 TOML 커맨드 파일과 마크다운 파일을 공용 `ResourceData`로 복원합니다.

## 수정 파일 (Files to Modify)
- `src/transformer/gemini.rs`

## 상세 지침 (Actionable Instructions)
1. `GeminiTransformer::detransform`을 구현합니다.
2. `ResourceType::Command`인 경우:
    - `file_content`를 `toml::from_str`로 파싱합니다.
    - `prompt` 필드를 추출하여 `content`로 설정합니다.
    - 나머지 필드(예: `description`, `model` 등)를 JSON `Value` 객체로 변환하여 `metadata`로 설정합니다. (단, `prompt`는 메타데이터에서 제거)
3. `ResourceType::Agent` 또는 `ResourceType::Skill`인 경우:
    - `DefaultTransformer`의 `detransform`을 호출하여 마크다운 본문과 Frontmatter를 추출합니다.

## 검증 방법 (Verification)
- `src/transformer/gemini.rs`에 `detransform` 테스트 케이스를 추가하여, TOML 파일이 `ResourceData`로 정확히 복원되는지 검증합니다.
- `description`과 `prompt` 필드의 추출 여부를 확인합니다.
