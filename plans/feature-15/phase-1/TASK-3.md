# TASK 3: `extract_frontmatter` 단위 테스트 추가 (멀티라인 파싱 확인)

## 상세 설명
`extract_frontmatter`가 멀티라인 YAML 블록(`|` 또는 `|-` 등)을 정확하게 파싱하여 데이터를 추출하는지 확인하는 테스트를 추가합니다.

## 수락 기준
- `|` 마커를 사용한 멀티라인 설명이 포함된 프론트매터 파싱 성공.
- 추출된 `metadata` 객체의 `description` 값이 개행 문자를 포함한 한 개의 문자열로 인식됨.
- 프론트매터 이후의 본문이 정확하게 분리됨.

## 테스트 예시
```rust
#[test]
fn test_extract_multiline_yaml() {
    let input = "---
description: |
  Line 1
  Line 2
name: test
---
Body content";
    let (meta, content) = extract_frontmatter(input);
    assert_eq!(meta["description"], "Line 1\nLine 2\n");
    assert_eq!(meta["name"], "test");
    assert_eq!(content, "Body content");
}
```
