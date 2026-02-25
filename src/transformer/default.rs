use crate::resource::{BuildTarget, Resource, TransformedFile};
use crate::transformer::Transformer;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct DefaultFrontmatter {
    metadata: serde_json::Value,
}

pub struct DefaultTransformer {
    pub target: BuildTarget,
}

impl Transformer for DefaultTransformer {
    fn transform(&self, resource: &Resource) -> Result<TransformedFile> {
        let (data, folder) = match resource {
            Resource::Command(d) => (d, "commands"),
            Resource::Agent(d) => (d, "agents"),
            Resource::Skill(d) => (d, "skills"),
        };

        let frontmatter = DefaultFrontmatter {
            metadata: data.metadata.clone(),
        };

        let yaml_frontmatter = serde_yaml::to_string(&frontmatter)?;
        let content = format!("---\n{}---\n\n{}", yaml_frontmatter, data.content);

        let path = if matches!(resource, Resource::Skill(_)) {
            PathBuf::from(folder).join(&data.name).join(format!("{}.md", data.name))
        } else {
            PathBuf::from(folder).join(format!("{}.md", data.name))
        };

        Ok(TransformedFile { path, content })
    }

    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile> {
        let filename = match self.target {
            BuildTarget::GeminiCli => "GEMINI.md",
            BuildTarget::ClaudeCode => "CLAUDE.md",
            BuildTarget::OpenCode => "OPENCODE.md",
        };

        Ok(TransformedFile {
            path: PathBuf::from(filename),
            content: content.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::ResourceData;
    use serde_json::json;

    #[test]
    fn test_default_transformation() {
        let transformer = DefaultTransformer {
            target: BuildTarget::ClaudeCode,
        };
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

        assert!(result.content.contains("metadata:"));
        assert!(result.content.contains("description: A test command"));
        assert!(result.content.contains("model: claude-3-opus"));
        assert!(result.content.contains("# Hello World"));
        assert!(result.content.starts_with("---"));
    }

    #[test]
    fn test_default_skill_transformation() {
        let transformer = DefaultTransformer {
            target: BuildTarget::ClaudeCode,
        };
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
        assert!(result.content.contains("metadata:"));
        assert!(result.content.contains("description: Skill description"));
        assert!(result.content.contains("type: expert"));
        assert!(result.content.contains("Skill Content"));
    }

    #[test]
    fn test_default_root_prompt_transformation() {
        let claude = DefaultTransformer {
            target: BuildTarget::ClaudeCode,
        };
        let opencode = DefaultTransformer {
            target: BuildTarget::OpenCode,
        };
        let gemini = DefaultTransformer {
            target: BuildTarget::GeminiCli,
        };

        assert_eq!(
            claude.transform_root_prompt("test").unwrap().path,
            PathBuf::from("CLAUDE.md")
        );
        assert_eq!(
            opencode.transform_root_prompt("test").unwrap().path,
            PathBuf::from("OPENCODE.md")
        );
        assert_eq!(
            gemini.transform_root_prompt("test").unwrap().path,
            PathBuf::from("GEMINI.md")
        );
    }
}
