# TASK 3-2: Content Sync Logic for Markdown

## 개요 (Description)
타겟에서 수정된 마크다운 본문(Content)이 소스 파일에 정확히 덮어씌워지도록 동기화 로직을 구현합니다.

## 수정 파일 (Files to Modify)
- `src/syncer/diff.rs` (신규 파일)

## 상세 지침 (Actionable Instructions)
1. `syncer::diff` 모듈을 생성합니다.
2. `diff_content(source: &str, target: &str) -> bool` 함수를 구현하여, 텍스트가 1글자라도 다르면 `true`를 반환합니다.
3. 소스 파일의 본문(Content) 부분을 타겟의 본문으로 교체하는 `replace_content(source: &str, new_content: &str) -> String` 함수를 구현합니다.
    - 이때 Frontmatter 부분은 `extract_frontmatter`를 통해 분리하여 유지하고, 순수 본문 부분만 교체한 뒤 재조합합니다.
4. 마크다운 본문 동기화 시 Frontmatter 영역과 본문 영역 사이의 빈 줄(Newlines) 규칙이 깨지지 않도록 유의합니다.

## 검증 방법 (Verification)
- `syncer::diff::tests`를 작성하여, 마크다운 본문 교체 로직의 정확성을 검증합니다.
- Frontmatter 영역이 유지되는지 확인합니다.
