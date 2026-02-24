use crate::resource::TransformedFile;
use crate::resource::resource::Resource;
use crate::transformers::base::Transformer;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct ClaudeFrontmatter {
    description: String,
    parameters: serde_json::Map<String, Value>,
}

pub struct ClaudeTransformer;

impl Transformer for ClaudeTransformer {
    fn transform(&self, resource: &Resource) -> Result<TransformedFile> {
        let (data, folder) = match resource {
            Resource::Command(d) => (d, "commands"),
            Resource::Agent(d) => (d, "agents"),
            Resource::Skill(d) => (d, "skills"),
        };

        // 1. JSON Metadata에서 description 추출 및 parameters 구성
        let mut metadata_obj = match data.metadata.as_object() {
            Some(obj) => obj.clone(),
            None => {
                return Err(anyhow!("Metadata must be a JSON object for Claude conversion"));
            }
        };

        let description = metadata_obj
            .remove("description")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default();

        let frontmatter = ClaudeFrontmatter {
            description,
            parameters: metadata_obj,
        };

        // 2. Frontmatter를 YAML로 직렬화
        let yaml_frontmatter = serde_yaml::to_string(&frontmatter)?;

        // 3. 내용 결합 (Frontmatter + Markdown)
        let content = format!("---\n{}---\n\n{}", yaml_frontmatter, data.content);

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
            path: PathBuf::from("CLAUDE.md"),
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
    fn test_claude_command_transformation() {
        let transformer = ClaudeTransformer;
        let resource = Resource::Command(ResourceData {
            name: "test-cmd".to_string(),
            plugin: "test-plugin".to_string(),
            content: "# Hello World".to_string(),
            metadata: json!({
                "description": "A test command",
                "model": "claude-3-opus"
            }),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(result.path, PathBuf::from("commands/test-cmd.md"));

        assert!(result.content.contains("description: A test command"));
        assert!(result.content.contains("parameters:"));
        assert!(result.content.contains("model: claude-3-opus"));
        assert!(result.content.contains("# Hello World"));
        assert!(result.content.starts_with("---"));
    }

    #[test]
    fn test_claude_skill_transformation() {
        let transformer = ClaudeTransformer;
        let resource = Resource::Skill(ResourceData {
            name: "test-skill".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Skill Content".to_string(),
            metadata: json!({
                "description": "Skill description",
                "type": "expert"
            }),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(result.path, PathBuf::from("skills/test-skill/test-skill.md"));
        assert!(result.content.contains("description: Skill description"));
        assert!(result.content.contains("type: expert"));
        assert!(result.content.contains("Skill Content"));
    }
}
