use crate::resource::TransformedFile;
use crate::resource::resource::Resource;
use crate::transformers::base::Transformer;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct OpenCodeFrontmatter {
    metadata: serde_json::Map<String, Value>,
}

pub struct OpenCodeTransformer;

impl Transformer for OpenCodeTransformer {
    fn transform(&self, resource: &Resource) -> Result<TransformedFile> {
        let (data, folder) = match resource {
            Resource::Command(d) => (d, "commands"),
            Resource::Agent(d) => (d, "agents"),
            Resource::Skill(d) => (d, "skills"),
        };

        // 1. JSON Metadata를 그대로 Frontmatter로 사용
        let metadata_obj = match data.metadata.as_object() {
            Some(obj) => obj.clone(),
            None => {
                return Err(anyhow!("Metadata must be a JSON object for OpenCode conversion"));
            }
        };

        let frontmatter = OpenCodeFrontmatter { metadata: metadata_obj };

        // 2. Frontmatter를 YAML로 직렬화
        let yaml_frontmatter = serde_yaml::to_string(&frontmatter)?;

        // 3. 내용 결합 (Frontmatter + Markdown)
        let content = format!(
            "---
{}---

{}",
            yaml_frontmatter, data.content
        );

        // 4. 경로 설정 (.md 확장자)
        let path = if matches!(resource, Resource::Skill(_)) {
            PathBuf::from(folder).join(&data.name).join(format!("{}.md", data.name))
        } else {
            PathBuf::from(folder).join(format!("{}.md", data.name))
        };

        Ok(TransformedFile { path, content })
    }

    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile> {
        Ok(TransformedFile {
            path: PathBuf::from("OPENCODE.md"),
            content: content.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::resource::ResourceData;
    use serde_json::json;

    #[test]
    fn test_opencode_transformation() {
        let transformer = OpenCodeTransformer;
        let resource = Resource::Command(ResourceData {
            name: "test-cmd".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Body Content".to_string(),
            metadata: json!({
                "key": "value"
            }),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(result.path, PathBuf::from("commands/test-cmd.md"));
        assert!(result.content.contains("metadata:"));
        assert!(result.content.contains("key: value"));
        assert!(result.content.contains("Body Content"));
    }
}
