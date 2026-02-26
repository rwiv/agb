# TASK 3-1: Implement Partial Update for Markdown `description`

## 개요 (Description)
원본 마크다운 파일의 포맷을 손상시키지 않고 `description` 필드만 업데이트하는 `update_description` 함수를 구현합니다.

## 수정 파일 (Files to Modify)
- `src/utils/yaml.rs` (또는 신규 `src/syncer/diff.rs`)

## 상세 지침 (Actionable Instructions)
1. `update_description(source: &str, new_desc: &str) -> String` 함수를 구현합니다.
2. 마크다운 본문의 Frontmatter 영역(`---`와 `---` 사이)을 식별합니다.
3. 해당 영역 내에서 `description:`으로 시작하는 줄을 찾습니다.
    - 찾았다면: 해당 줄을 `description: <new_desc>`로 교체합니다. 이때 기존 주석이나 들여쓰기를 유지할 수 있도록 정규표현식(`^(\s*description:\s*).*$`)을 사용하는 것이 안전합니다.
    - 찾지 못했다면: Frontmatter 끝(`---`) 직전에 `description: <new_desc>` 줄을 추가합니다.
4. 업데이트된 Frontmatter와 나머지 본문을 재조합하여 반환합니다.

## 검증 방법 (Verification)
- 다양한 마크다운 파일(주석 포함, 여러 줄 등)에 대해 유닛 테스트를 작성하여 포맷 유지 여부를 검증합니다.
- `description` 외의 다른 필드가 변하지 않는지 확인합니다.
