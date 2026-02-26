# TASK 5-1: Unit Tests for Partial Update and Detransform

## 개요 (Description)
`Syncer`의 핵심 로직인 `Partial Update`와 `Detransform`에 대한 철저한 유닛 테스트를 작성합니다.

## 수정 파일 (Files to Modify)
- `src/utils/yaml.rs`
- `src/transformer/gemini.rs`
- `src/transformer/default.rs`

## 상세 지침 (Actionable Instructions)
1. `Partial Update` (`update_description`)에 대해 다음 케이스를 테스트합니다.
    - `description` 필드가 이미 존재하는 경우 (수정 여부)
    - `description` 필드가 없는 경우 (추가 여부)
    - `description` 줄에 주석이 있는 경우 (주석 보존 여부)
    - Frontmatter 영역이 비어있는 경우
2. `Detransform`에 대해 다음 케이스를 테스트합니다.
    - Gemini TOML에서 `prompt`와 `description`이 정확히 분리되는지
    - 마크다운 Frontmatter에서 특정 필드만 추출되는지
    - 잘못된 형식의 파일에 대해 적절한 에러를 반환하는지

## 검증 방법 (Verification)
- `cargo test transformer` 및 `cargo test utils`를 실행하여 모든 테스트가 통과하는지 확인합니다.
