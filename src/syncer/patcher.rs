use regex::Regex;

pub struct MdPatcher {
    raw_content: String,
}

impl MdPatcher {
    pub fn new(content: &str) -> Self {
        Self {
            raw_content: content.to_string(),
        }
    }

    /// description 필드만 업데이트 (기존 update_description 로직)
    pub fn update_description(&mut self, new_desc: &str) {
        let content = self.raw_content.trim_start();
        if !content.starts_with("---") {
            // Frontmatter가 없는 경우 새로 생성
            self.raw_content = format!("---\ndescription: {}\n---\n\n{}", new_desc, self.raw_content);
            return;
        }

        let rest = &content[3..];
        if let Some(end_offset) = rest.find("---") {
            let yaml_part = &rest[..end_offset];
            let pure_content = &rest[end_offset + 3..];

            let mut lines: Vec<String> = yaml_part.lines().map(|s| s.to_string()).collect();
            let mut found = false;

            // description: 키를 찾아 교체 (공백 및 인용부호 허용)
            let re = Regex::new(r#"^(\s*description:\s*)(?:'[^']*'|"[^"]*"|.*)$"#).unwrap();
            for line in lines.iter_mut() {
                if let Some(caps) = re.captures(line) {
                    let prefix = caps.get(1).unwrap().as_str();
                    *line = format!("{}{}", prefix, new_desc);
                    found = true;
                    break;
                }
            }

            if !found {
                // 못 찾았다면 마지막에 추가
                lines.push(format!("description: {}", new_desc));
            }

            self.raw_content = format!("---\n{}\n---{}", lines.join("\n"), pure_content);
        } else {
            // 닫는 ---가 없는 경우 (잘못된 형식), 안전하게 앞에 추가
            self.raw_content = format!("---\ndescription: {}\n---\n\n{}", new_desc, self.raw_content);
        }
    }

    /// 본문 영역만 교체 (기존 replace_content 로직)
    pub fn replace_body(&mut self, new_body: &str) {
        let content = self.raw_content.trim_start();
        if !content.starts_with("---") {
            // Frontmatter가 없는 경우 그냥 덮어씀 (단, Frontmatter가 없으므로 new_body만)
            self.raw_content = new_body.to_string();
            return;
        }

        let rest = &content[3..];
        if let Some(end_offset) = rest.find("---") {
            let yaml_part = &rest[..end_offset];
            // Frontmatter 영역을 유지하고 본문만 교체 (--- 뒤에 개행 추가 보장)
            self.raw_content = format!("---\n{}\n---\n\n{}", yaml_part.trim_end(), new_body.trim_start());
        } else {
            // 닫는 ---가 없는 경우 그냥 덮어씀
            self.raw_content = new_body.to_string();
        }
    }

    /// 최종 마크다운 문자열 렌더링
    pub fn render(&self) -> String {
        self.raw_content.clone()
    }

    /// 텍스트가 1글자라도 다르면 true를 반환합니다. (기존 diff_content 로직)
    pub fn has_changed(&self, other: &str) -> bool {
        self.raw_content.trim() != other.trim()
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
        assert!(!patcher.has_changed("hello\n"));
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
        patcher.update_description("new description");
        let updated = patcher.render();
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
        patcher.update_description("\"new quoted description\"");
        let updated = patcher.render();
        assert!(updated.contains("description: \"new quoted description\""));
    }

    #[test]
    fn test_update_description_with_comments() {
        let source = "---
name: test # name comment
description: old # desc comment
# overall comment
---";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new");
        let updated = patcher.render();
        assert!(updated.contains("name: test # name comment"));
        assert!(updated.contains("description: new"));
        assert!(updated.contains("# overall comment"));
    }

    #[test]
    fn test_replace_body_preserves_frontmatter_exactly() {
        let source = "---
name: test
# comment
description: desc
---
# Old Body";
        let mut patcher = MdPatcher::new(source);
        patcher.replace_body("# New Body");
        let updated = patcher.render();
        assert!(updated.contains("name: test"));
        assert!(updated.contains("# comment"));
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
        patcher.update_description("new description");
        let updated = patcher.render();
        assert!(updated.contains("description: new description"));
        assert!(updated.contains("name: test"));
    }

    #[test]
    fn test_update_description_no_frontmatter() {
        let source = "# Content";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new description");
        let updated = patcher.render();
        assert!(updated.contains("description: new description"));
        assert!(updated.contains("# Content"));
        assert!(updated.starts_with("---"));
    }

    #[test]
    fn test_patch_empty_source() {
        let source = "";
        let mut patcher = MdPatcher::new(source);
        patcher.update_description("new");
        patcher.replace_body("# New Body");
        let updated = patcher.render();
        assert!(updated.contains("description: new"));
        assert!(updated.contains("# New Body"));
    }
}
