use crate::core::{BuildTarget, DIR_COMMANDS, EXT_TOML, GEMINI_MD, Resource, ResourceData, TransformedFile};
use crate::transformer::Transformer;
use crate::transformer::default::DefaultTransformer;
use crate::utils::toml::json_to_toml;
use anyhow::{Result, anyhow};
use std::path::PathBuf;

pub struct GeminiTransformer;

impl Transformer for GeminiTransformer {
    fn transform(&self, resource: &Resource) -> Result<TransformedFile> {
        match resource {
            Resource::Command(data) => self.transform_command_to_toml(data),
            Resource::Agent(_) | Resource::Skill(_) => {
                let default_transformer = DefaultTransformer {
                    target: BuildTarget::GeminiCli,
                };
                default_transformer.transform(resource)
            }
        }
    }

    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile> {
        // AGENTS.md -> GEMINI.md
        Ok(TransformedFile {
            path: PathBuf::from(GEMINI_MD),
            content: content.to_string(),
        })
    }
}

impl GeminiTransformer {
    fn transform_command_to_toml(&self, data: &ResourceData) -> Result<TransformedFile> {
        // 1. Metadata를 TOML Value로 변환 후 Table로 캐스팅
        let json_value = &data.metadata;
        let toml_value = json_to_toml(json_value)?;

        let mut table = match toml_value {
            toml::Value::Table(t) => t,
            _ => {
                return Err(anyhow!("Metadata must be an object for Gemini conversion"));
            }
        };

        // 2. Markdown content를 'prompt' 필드에 추가
        let mut prompt = data.content.clone();
        if prompt.contains('\n') && !prompt.ends_with('\n') {
            prompt.push('\n');
        }
        table.insert("prompt".to_string(), toml::Value::String(prompt));

        // 3. TOML 직렬화
        let content = toml::to_string_pretty(&table)?;

        // 4. 경로 설정
        let path = PathBuf::from(DIR_COMMANDS).join(format!("{}{}", data.name, EXT_TOML));

        Ok(TransformedFile { path, content })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ResourceData;
    use serde_json::json;
    use toml::Table;

    #[test]
    fn test_gemini_command_transformation() {
        let transformer = GeminiTransformer;
        let resource = Resource::Command(ResourceData {
            name: "test-cmd".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Hello World".to_string(),
            metadata: json!({
                "model": "gemini-1.5-pro",
                "description": "A test command"
            }),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(
            result.path,
            PathBuf::from(DIR_COMMANDS).join(format!("test-cmd{}", EXT_TOML))
        );

        let toml_val: Table = toml::from_str(&result.content).unwrap();
        assert_eq!(toml_val.get("model").unwrap().as_str().unwrap(), "gemini-1.5-pro");
        assert_eq!(toml_val.get("description").unwrap().as_str().unwrap(), "A test command");
        assert_eq!(toml_val.get("prompt").unwrap().as_str().unwrap(), "Hello World");
    }

    #[test]
    fn test_gemini_command_multiline_prompt_formatting() {
        let transformer = GeminiTransformer;
        let resource = Resource::Command(ResourceData {
            name: "multiline-cmd".to_string(),
            plugin: "test-plugin".to_string(),
            content: "line1\nline2".to_string(),
            metadata: json!({}),
        });

        let result = transformer.transform(&resource).unwrap();
        // Ensure the closing triple quotes are on a new line
        assert!(result.content.contains("line2\n\"\"\""));
    }

    #[test]
    fn test_gemini_skill_transformation() {
        let transformer = GeminiTransformer;
        let resource = Resource::Skill(ResourceData {
            name: "test-skill".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Skill Content".to_string(),
            metadata: json!({
                "type": "expert"
            }),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(
            result.path,
            PathBuf::from(crate::core::DIR_SKILLS)
                .join("test-skill")
                .join(crate::core::SKILL_MD)
        );
        assert!(result.content.contains("metadata:"));
        assert!(result.content.contains("type: expert"));
        assert!(result.content.contains("Skill Content"));
    }

    #[test]
    fn test_gemini_agent_transformation() {
        let transformer = GeminiTransformer;
        let resource = Resource::Agent(ResourceData {
            name: "test-agent".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Agent Content".to_string(),
            metadata: json!({
                "model": "gemini-1.5-flash"
            }),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(
            result.path,
            PathBuf::from(crate::core::DIR_AGENTS).join(format!("test-agent{}", crate::core::EXT_MD))
        );
        assert!(result.content.contains("metadata:"));
        assert!(result.content.contains("model: gemini-1.5-flash"));
        assert!(result.content.contains("Agent Content"));
    }

    #[test]
    fn test_gemini_root_prompt_transformation() {
        let transformer = GeminiTransformer;
        let content = "# Global Instructions\nDo this and that.";
        let result = transformer.transform_root_prompt(content).unwrap();

        assert_eq!(result.path, PathBuf::from(GEMINI_MD));
        assert_eq!(result.content, content);
    }
}
