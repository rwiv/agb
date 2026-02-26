# TASK 2-3: Implement `detransform` for `DefaultTransformer`

## 개요 (Description)
`DefaultTransformer`의 `detransform` 로직을 구현하여 마크다운 파일(`.md`)로부터 `ResourceData`를 복원합니다.

## 수정 파일 (Files to Modify)
- `src/transformer/default.rs`

## 상세 지침 (Actionable Instructions)
1. `DefaultTransformer::detransform`을 구현합니다.
2. `utils::yaml::extract_frontmatter`를 활용하여 마크다운 본문과 Frontmatter를 분리합니다.
3. 분리된 Frontmatter는 `metadata`로, 본문은 `content`로 설정하여 `ResourceData` 객체를 생성합니다.
4. 모든 리소스 타입(`Command`, `Agent`, `Skill`)에 대해 공통적으로 이 로직이 적용되는지 확인합니다.

## 검증 방법 (Verification)
- `src/transformer/default.rs`에 `detransform` 테스트 케이스를 추가하여, 마크다운 파일이 `ResourceData`로 정확히 복원되는지 검증합니다.
- `description` 필드가 메타데이터 내에 포함되는지 확인합니다.
