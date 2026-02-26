# Task 1-1: `MarkdownPatcher` 구현

## 1. 개요
마크다운 파일의 구조를 유지하며 특정 부분만 수정하는 `MarkdownPatcher` 구조체를 구현합니다. 이는 기존 `src/syncer/diff.rs`에 흩어져 있던 텍스트 조작 함수들을 하나로 모으고 객체지향적으로 개선하는 작업입니다.

## 2. 작업 상세

### 2.1. 파일 생성
- `src/syncer/diff/markdown.rs` 파일을 생성합니다.

### 2.2. `MarkdownPatcher` 구조체 정의
```rust
pub struct MarkdownPatcher {
    raw_content: String,
    // 필요 시 Frontmatter와 Body를 미리 분리하여 보관 가능
}

impl MarkdownPatcher {
    pub fn new(content: &str) -> Self { ... }
    
    /// description 필드만 업데이트 (기존 update_description 로직)
    pub fn update_description(&mut self, new_desc: &str) { ... }
    
    /// 본문 영역만 교체 (기존 replace_content 로직)
    pub fn replace_body(&mut self, new_body: &str) { ... }
    
    /// 최종 마크다운 문자열 렌더링
    pub fn render(&self) -> String { ... }
}
```

### 2.3. 단위 테스트 작성
- 기존 `diff.rs`에 있던 테스트 케이스들을 이동시키고, `MarkdownPatcher` 구조체에 맞춰 수정하여 추가합니다.

## 3. 검증 방법
- `cargo test syncer::diff::markdown` 명령어를 통해 모든 테스트가 통과하는지 확인합니다.
