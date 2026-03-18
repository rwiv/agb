use crate::utils::yaml::extract_frontmatter;

pub struct MdPatcher {
    raw_content: String,
}

impl MdPatcher {
    pub fn new(content: &str) -> Self {
        Self {
            raw_content: content.to_string(),
        }
    }

    /// description 필드만 업데이트 (YAML 기반 로직)
    pub fn update_description(&mut self, new_desc: &str) -> anyhow::Result<()> {
        // 1. 프론트매터와 본문 분리
        let (mut metadata, body) = extract_frontmatter(&self.raw_content);

        // 2. description 필드 업데이트
        if let Some(obj) = metadata.as_object_mut() {
            obj.insert(
                "description".to_string(),
                serde_json::Value::String(new_desc.to_string()),
            );
        } else {
            // 메타데이터가 객체가 아닌 경우 (비어있는 경우 등) 새로 생성
            metadata = serde_json::json!({ "description": new_desc });
        }

        // 3. YAML로 직렬화
        let new_yaml = serde_yaml::to_string(&metadata)?;
        // serde_yaml이 생성하는 --- 및 ... 제거
        let new_yaml = new_yaml.trim_start_matches("---").trim_end_matches("...").trim();

        // 4. 본문과 재결합
        self.raw_content = format!("---\n{}\n---\n\n{}", new_yaml, body);

        Ok(())
    }

    /// 본문 영역만 교체 (기존 replace_content 로직)
    pub fn replace_body(&mut self, new_body: &str) {
        let content = self.raw_content.trim_start();
        if !content.starts_with("---") {
            self.raw_content = new_body.to_string();
            return;
        }

        let rest = &content[3..];
        if let Some(end_offset) = rest.find("---") {
            let yaml_part = rest[..end_offset].trim();
            // Frontmatter 영역을 유지하고 본문만 교체
            // new_body 앞에 개행 문자가 중복되는 것을 방지하기 위해 trim_start_matches 사용
            let new_body = new_body.trim_start_matches(['\r', '\n']);
            self.raw_content = format!("---\n{}\n---\n\n{}", yaml_part, new_body);
        } else {
            self.raw_content = new_body.to_string();
        }
    }

    /// 최종 마크다운 문자열 렌더링
    pub fn render(&self) -> String {
        self.raw_content.clone()
    }

    /// 본문 영역만 추출합니다.
    fn get_body(&self) -> &str {
        let content = self.raw_content.trim_start();
        if !content.starts_with("---") {
            return content;
        }

        let rest = &content[3..];
        if let Some(end_offset) = rest.find("---") {
            let pure_content = &rest[end_offset + 3..];
            pure_content.trim_start_matches(['\r', '\n'])
        } else {
            content
        }
    }

    /// 텍스트가 1글자라도 다르면 true를 반환합니다.
    pub fn has_changed(&self, other: &str) -> bool {
        self.get_body() != other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_changed() {
        let patcher = MdPatcher::new("hello");
        assert!(patcher.has_changed("world"));
        assert!(!patcher.has_changed("hello"));
        assert!(patcher.has_changed("hello\n"));
    }

    #[test]
    fn test_replace_body() {
        let source = "---
name: test
---
# Old Content";
        let mut patcher = MdPatcher::new(source);
        patcher.replace_body("# New Content");
        let updated = patcher.render();
        assert!(updated.contains("name: test"));
        assert!(updated.contains("# New Content"));
        assert!(!updated.contains("# Old Content"));
    }

    #[test]
    fn test_update_description_existing() {
        let source = "---
name: test
description: old description
---
# Content";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new description").unwrap();
        let updated = patcher.render();
        // YAML 파서에 의해 필드 순서가 바뀔 수 있으므로 렌더링 결과 포함 여부로 확인
        assert!(updated.contains("description: new description"));
        assert!(updated.contains("name: test"));
        assert!(updated.contains("# Content"));
    }

    #[test]
    fn test_update_description_quoted() {
        let source = "---
description: 'old quoted description'
---";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new quoted description").unwrap();
        let updated = patcher.render();
        // serde_yaml은 필요한 경우 따옴표를 자동으로 처리함
        assert!(updated.contains("description: new quoted description"));
    }

    #[test]
    fn test_update_description_multiline_input() {
        let source = "---\nname: test\n---";
        let mut patcher = MdPatcher::new(source);
        let new_desc = "Line 1\nLine 2";
        patcher.update_description(new_desc).unwrap();
        let updated = patcher.render();
        // YAML 멀티라인 마커 (|- 등) 포함 확인
        assert!(updated.contains("description: |-"));
        assert!(updated.contains("Line 1"));
        assert!(updated.contains("Line 2"));
    }

    #[test]
    fn test_update_description_from_multiline_to_singleline() {
        let source = "---\ndescription: |\n  Line 1\n  Line 2\n---";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new single line").unwrap();
        let updated = patcher.render();
        assert!(updated.contains("description: new single line"));
        assert!(!updated.contains("Line 1"));
    }

    #[test]
    fn test_update_description_preserves_other_fields() {
        let source = "---
name: my-agent
model: gpt-4
description: old
---";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new").unwrap();
        let updated = patcher.render();
        assert!(updated.contains("name: my-agent"));
        assert!(updated.contains("model: gpt-4"));
        assert!(updated.contains("description: new"));
    }

    #[test]
    fn test_replace_body_preserves_frontmatter_exactly() {
        let source = "---
name: test
description: desc
---
# Old Body";
        let mut patcher = MdPatcher::new(source);
        patcher.replace_body("# New Body");
        let updated = patcher.render();
        assert!(updated.contains("name: test"));
        assert!(updated.contains("description: desc"));
        assert!(updated.contains("# New Body"));
        assert!(updated.contains("---\n\n# New Body")); // 개행 보장 확인
    }

    #[test]
    fn test_update_description_missing() {
        let source = "---
name: test
---
# Content";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new description").unwrap();
        let updated = patcher.render();
        assert!(updated.contains("description: new description"));
        assert!(updated.contains("name: test"));
    }

    #[test]
    fn test_update_description_no_frontmatter() {
        let source = "# Content";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new description").unwrap();
        let updated = patcher.render();
        assert!(updated.contains("description: new description"));
        assert!(updated.contains("# Content"));
        assert!(updated.starts_with("---"));
    }

    #[test]
    fn test_patch_empty_source() {
        let source = "";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new").unwrap();
        patcher.replace_body("# New Body");
        let updated = patcher.render();
        assert!(updated.contains("description: new"));
        assert!(updated.contains("# New Body"));
    }

    #[test]
    fn test_newline_accumulation_prevention() {
        let source = "---\nname: test\ndescription: old\n---\n\n# Body";
        let mut patcher = MdPatcher::new(source);

        // 여러 번 업데이트 수행
        patcher.update_description("new1").unwrap();
        patcher.update_description("new2").unwrap();
        patcher.replace_body("# New Body");
        patcher.update_description("new3").unwrap();

        let updated = patcher.render();
        println!("DEBUG UPDATED:\n{}", updated);

        // 필드 존재 여부 확인 (순서는 바뀔 수 있음)
        assert!(updated.contains("name: test"));
        assert!(updated.contains("description: new3"));
        assert!(updated.contains("# New Body"));

        // "---" 가 정확히 두 번 나타나는지 확인 (중첩 방지)
        assert_eq!(updated.matches("---").count(), 2);
    }
}
