# TASK 2: `MdPatcher` 단위 테스트 추가 (멀티라인 지원 확인)

## 상세 설명
`MdPatcher::update_description`에 멀티라인 입력이 주어졌을 때와 원본이 멀티라인인 경우에 대한 테스트 케이스를 작성합니다.

## 수락 기준
- 다음 시나리오에 대한 단위 테스트 통과:
    - 한 줄 `description`을 멀티라인으로 업데이트.
    - 멀티라인 `description`을 한 줄로 업데이트.
    - 멀티라인 `description`을 다른 멀티라인으로 업데이트.
    - `description`이 없는 프론트매터에 멀티라인 설명 추가.
    - 프론트매터가 아예 없는 파일에 멀티라인 설명 추가.
    - 업데이트 후에도 다른 메타데이터 필드들이 정확히 보존되는지 확인.

## 테스트 예시
```rust
#[test]
fn test_update_description_multiline_input() {
    let source = "---\nname: test\n---";
    let mut patcher = MdPatcher::new(source);
    let new_desc = "Line 1\nLine 2";
    patcher.update_description(new_desc).unwrap();
    let updated = patcher.render();
    assert!(updated.contains("description: |-")); // YAML 멀티라인 마커 확인
    assert!(updated.contains("Line 1"));
    assert!(updated.contains("Line 2"));
}
```
