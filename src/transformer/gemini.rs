use crate::resource::{BuildTarget, Resource, TransformedFile};
use crate::transformer::Transformer;
use crate::transformer::default::DefaultTransformer;
use anyhow::{Result, anyhow};
use serde_json::Value;
use std::path::PathBuf;
use toml::Table;

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
            path: PathBuf::from("GEMINI.md"),
            content: content.to_string(),
        })
    }
}

impl GeminiTransformer {
    fn transform_command_to_toml(&self, data: &crate::resource::ResourceData) -> Result<TransformedFile> {
        // 1. JSON Metadata를 TOML Value로 변환 후 Table로 캐스팅
        let json_value = &data.metadata;
        let toml_value = json_to_toml(json_value)?;

        let mut table = match toml_value {
            toml::Value::Table(t) => t,
            _ => {
                return Err(anyhow!("Metadata must be a JSON object for Gemini conversion"));
            }
        };

        // 2. Markdown content를 'prompt' 필드에 추가
        table.insert("prompt".to_string(), toml::Value::String(data.content.clone()));

        // 3. TOML 직렬화
        let content = toml::to_string_pretty(&table)?;

        // 4. 경로 설정
        let path = PathBuf::from("commands").join(format!("{}.toml", data.name));

        Ok(TransformedFile { path, content })
    }
}

fn json_to_toml(value: &serde_json::Value) -> Result<toml::Value> {
    match value {
        Value::Null => Ok(toml::Value::Table(Table::new())),
        Value::Bool(b) => Ok(toml::Value::Boolean(*b)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(toml::Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(toml::Value::Float(f))
            } else {
                Err(anyhow!("Invalid number in metadata"))
            }
        }
        Value::String(s) => Ok(toml::Value::String(s.clone())),
        Value::Array(arr) => {
            let mut toml_arr = Vec::new();
            for v in arr {
                toml_arr.push(json_to_toml(v)?);
            }
            Ok(toml::Value::Array(toml_arr))
        }
        Value::Object(obj) => {
            let mut table = Table::new();
            for (k, v) in obj {
                table.insert(k.clone(), json_to_toml(v)?);
            }
            Ok(toml::Value::Table(table))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::ResourceData;
    use serde_json::json;

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
        assert_eq!(result.path, PathBuf::from("commands/test-cmd.toml"));

        let toml_val: Table = toml::from_str(&result.content).unwrap();
        assert_eq!(toml_val.get("model").unwrap().as_str().unwrap(), "gemini-1.5-pro");
        assert_eq!(toml_val.get("description").unwrap().as_str().unwrap(), "A test command");
        assert_eq!(toml_val.get("prompt").unwrap().as_str().unwrap(), "Hello World");
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
        assert_eq!(result.path, PathBuf::from("skills/test-skill/test-skill.md"));
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
        assert_eq!(result.path, PathBuf::from("agents/test-agent.md"));
        assert!(result.content.contains("metadata:"));
        assert!(result.content.contains("model: gemini-1.5-flash"));
        assert!(result.content.contains("Agent Content"));
    }

    #[test]
    fn test_gemini_root_prompt_transformation() {
        let transformer = GeminiTransformer;
        let content = "# Global Instructions\nDo this and that.";
        let result = transformer.transform_root_prompt(content).unwrap();

        assert_eq!(result.path, PathBuf::from("GEMINI.md"));
        assert_eq!(result.content, content);
    }
}
