# TASK 1: `MdPatcher::update_description` 구현 수정 (YAML 기반)

## 상세 설명
기존의 정규표현식 및 라인 단위 매칭 기반 `description` 업데이트 로직을 `serde_yaml`을 사용하는 방식으로 변경합니다. 100% 포맷 보존 대신 데이터의 정확한 처리에 집중합니다.

## 수락 기준
- `MdPatcher::update_description`이 `new_desc`로 들어오는 멀티라인 문자열을 정상적으로 처리함.
- 프론트매터 내의 기존 `description` 필드가 멀티라인인 경우에도 정확하게 업데이트됨.
- `description` 필드가 누락된 경우 새로 추가하며, 프론트매터 자체가 없는 경우에도 생성함.
- 마크다운 본문은 1바이트의 오차도 없이 보존됨.

## 구현 상세

1.  **프론트매터 분리**: `extract_frontmatter`를 활용하여 YAML 부분과 본문을 분리합니다.
2.  **YAML 파싱**: `serde_yaml::from_str`을 사용하여 메타데이터 객체(`serde_json::Value`)를 생성합니다.
3.  **필드 업데이트**: `metadata` 객체의 `description` 키 값을 새로운 값으로 교체합니다.
4.  **YAML 직렬화**: `serde_yaml::to_string`을 사용하여 업데이트된 메타데이터를 다시 YAML 문자열로 변환합니다.
5.  **재구성**: 새로운 YAML 프론트매터와 원본 본문을 결합하여 최종 내용을 생성합니다.

```rust
pub fn update_description(&mut self, new_desc: &str) -> anyhow::Result<()> {
    // 1. 프론트매터와 본문 분리
    let (mut metadata, body) = extract_frontmatter(&self.raw_content);

    // 2. description 필드 업데이트
    if let Some(obj) = metadata.as_object_mut() {
        obj.insert("description".to_string(), serde_json::Value::String(new_desc.to_string()));
    } else {
        metadata = serde_json::json!({ "description": new_desc });
    }

    // 3. YAML로 직렬화 (--- 와 ... 제거 등 정규화 필요)
    let new_yaml = serde_yaml::to_string(&metadata)?;
    let new_yaml = new_yaml.trim_start_matches("---").trim_end_matches("...").trim();

    // 4. 본문과 재결합
    self.raw_content = format!("---\n{}\n---\n\n{}", new_yaml, body);

    Ok(())
}
```
