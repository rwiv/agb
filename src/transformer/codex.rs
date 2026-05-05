use crate::core::{
    AGENTS_MD, BuildTarget, CODEX_CONFIG_FILE_NAME, CODEX_OPENAI_POLICY_RELATIVE_PATH, DIR_AGENTS, DIR_AGENTS_SKILLS,
    EXT_TOML, Resource, ResourceData, ResourceType, SKILL_MD, SkillData, TransformedFile,
};
use crate::transformer::Transformer;
use crate::transformer::default::DefaultTransformer;
use crate::utils::toml::json_to_toml;
use anyhow::{Result, anyhow};
use serde_json::Value;
use std::path::{Path, PathBuf};

pub struct CodexTransformer;

const DISABLE_MODEL_INVOCATION: &str = "disable-model-invocation";
const OPENAI_POLICY_CONTENT: &str = "policy:\n  allow_implicit_invocation: false\n";

impl Transformer for CodexTransformer {
    fn transform(&self, resource: &Resource) -> Result<TransformedFile> {
        match resource {
            Resource::Command(data) => {
                let default_transformer = DefaultTransformer {
                    target: BuildTarget::Codex,
                };
                let mut transformed = default_transformer.transform(resource)?;
                transformed.path = PathBuf::from(DIR_AGENTS_SKILLS).join(&data.name).join(SKILL_MD);
                Ok(transformed)
            }
            Resource::Agent(data) => self.transform_agent_to_toml(data),
            Resource::Skill(data) => {
                let default_transformer = DefaultTransformer {
                    target: BuildTarget::Codex,
                };
                let mut transformed = default_transformer.transform(resource)?;
                transformed.path = PathBuf::from(DIR_AGENTS_SKILLS).join(&data.base.name).join(SKILL_MD);
                Ok(transformed)
            }
        }
    }

    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile> {
        // AGENTS.md
        Ok(TransformedFile {
            path: PathBuf::from(AGENTS_MD),
            content: content.to_string(),
        })
    }

    fn post_transform(&self, resources: &[&Resource]) -> Result<Vec<TransformedFile>> {
        let mut files = Vec::new();

        if let Some(config_file) = self.transform_agent_registry(resources)? {
            files.push(config_file);
        }
        files.extend(self.transform_openai_policy_files(resources));

        Ok(files)
    }

    fn generated_extra_ignore_paths(&self, resource: &Resource) -> Vec<PathBuf> {
        let Resource::Skill(skill) = resource else {
            return Vec::new();
        };

        if requires_openai_policy(&skill.base.metadata) && !has_source_openai_policy(skill) {
            return vec![PathBuf::from(CODEX_OPENAI_POLICY_RELATIVE_PATH)];
        }

        Vec::new()
    }

    fn detransform(
        &self,
        r_type: ResourceType,
        name: &str,
        file_content: &str,
        output_dir: &std::path::Path,
    ) -> Result<ResourceData> {
        match r_type {
            ResourceType::Command => {
                let default_transformer = DefaultTransformer {
                    target: BuildTarget::Codex,
                };
                default_transformer.detransform(r_type, name, file_content, output_dir)
            }
            ResourceType::Agent => {
                // agents/ 내의 .toml 파일을 ResourceData로 복원
                let mut table: toml::Table = toml::from_str(file_content)?;
                let prompt = table
                    .remove("developer_instructions")
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .ok_or_else(|| anyhow!("Missing 'developer_instructions' field in Codex agent TOML"))?;

                let mut metadata = serde_json::to_value(table)?;

                // config.toml을 파싱하여 description을 복원합니다.
                let config_path = output_dir.join(CODEX_CONFIG_FILE_NAME);
                if config_path.exists()
                    && let Ok(config_content) = std::fs::read_to_string(&config_path)
                    && let Ok(config_table) = toml::from_str::<toml::Table>(&config_content)
                    && let Some(agents) = config_table.get("agents").and_then(|a| a.as_table())
                    && let Some(agent) = agents.get(name).and_then(|a| a.as_table())
                    && let Some(desc) = agent.get("description").and_then(|d| d.as_str())
                    && let Some(obj) = metadata.as_object_mut()
                {
                    obj.insert("description".to_string(), serde_json::Value::String(desc.to_string()));
                }

                Ok(ResourceData {
                    name: name.to_string(),
                    plugin: String::new(),
                    content: prompt,
                    metadata,
                    source_path: PathBuf::new(),
                })
            }
            ResourceType::Skill => {
                let default_transformer = DefaultTransformer {
                    target: BuildTarget::Codex,
                };
                default_transformer.detransform(r_type, name, file_content, output_dir)
            }
        }
    }

    fn get_target_path(&self, r_type: ResourceType, name: &str) -> PathBuf {
        match r_type {
            ResourceType::Command => PathBuf::from(DIR_AGENTS_SKILLS).join(name).join(SKILL_MD),
            ResourceType::Agent => PathBuf::from(DIR_AGENTS).join(format!("{}{}", name, EXT_TOML)),
            ResourceType::Skill => PathBuf::from(DIR_AGENTS_SKILLS).join(name).join(SKILL_MD),
        }
    }
}

impl CodexTransformer {
    fn transform_agent_registry(&self, resources: &[&Resource]) -> Result<Option<TransformedFile>> {
        let mut agents_table = toml::Table::new();

        for res in resources {
            let Resource::Agent(data) = res else {
                continue;
            };

            let mut agent_config = toml::Table::new();

            let description = data
                .metadata
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            agent_config.insert("description".to_string(), toml::Value::String(description));
            agent_config.insert(
                "config_file".to_string(),
                toml::Value::String(format!("agents/{}{}", data.name, EXT_TOML)),
            );

            agents_table.insert(data.name.clone(), toml::Value::Table(agent_config));
        }

        if agents_table.is_empty() {
            return Ok(None);
        }

        let mut root_table = toml::Table::new();
        root_table.insert("agents".to_string(), toml::Value::Table(agents_table));

        let content = toml::to_string_pretty(&root_table)?;
        let path = PathBuf::from(CODEX_CONFIG_FILE_NAME);

        Ok(Some(TransformedFile { path, content }))
    }

    fn transform_openai_policy_files(&self, resources: &[&Resource]) -> Vec<TransformedFile> {
        resources
            .iter()
            .filter_map(|resource| match resource {
                Resource::Command(data) if requires_openai_policy(&data.metadata) => {
                    Some(openai_policy_file(&data.name))
                }
                Resource::Skill(skill)
                    if requires_openai_policy(&skill.base.metadata) && !has_source_openai_policy(skill) =>
                {
                    Some(openai_policy_file(&skill.base.name))
                }
                _ => None,
            })
            .collect()
    }

    fn transform_agent_to_toml(&self, data: &ResourceData) -> Result<TransformedFile> {
        // Metadata를 TOML Value로 변환 후 Table로 캐스팅
        let json_value = &data.metadata;
        let toml_value = json_to_toml(json_value)?;

        let mut table = match toml_value {
            toml::Value::Table(mut t) => {
                // description은 config.toml로 분리되므로 개별 파일에서는 제거합니다.
                t.remove("description");
                t
            }
            _ => {
                return Err(anyhow!("Metadata must be an object for Codex agent conversion"));
            }
        };

        // Markdown content를 'developer_instructions' 필드에 추가
        let mut prompt = data.content.clone();
        if prompt.contains('\n') && !prompt.ends_with('\n') {
            prompt.push('\n');
        }
        table.insert("developer_instructions".to_string(), toml::Value::String(prompt));

        // TOML 직렬화
        let content = toml::to_string_pretty(&table)?;

        // 경로 설정: agents/[name].toml
        let path = PathBuf::from(DIR_AGENTS).join(format!("{}{}", data.name, EXT_TOML));

        Ok(TransformedFile { path, content })
    }
}

fn requires_openai_policy(metadata: &Value) -> bool {
    metadata.get(DISABLE_MODEL_INVOCATION).and_then(|value| value.as_bool()) == Some(true)
}

fn has_source_openai_policy(skill: &SkillData) -> bool {
    skill.extras.iter().any(|extra| {
        extra
            .source
            .strip_prefix(&skill.base.source_path)
            .map(|relative| relative == Path::new(CODEX_OPENAI_POLICY_RELATIVE_PATH))
            .unwrap_or(false)
    })
}

fn openai_policy_file(name: &str) -> TransformedFile {
    TransformedFile {
        path: PathBuf::from(DIR_AGENTS_SKILLS)
            .join(name)
            .join(CODEX_OPENAI_POLICY_RELATIVE_PATH),
        content: OPENAI_POLICY_CONTENT.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ExtraFile, ResourceData, SkillData};
    use serde_json::json;
    use toml::Table;

    fn policy_path(name: &str) -> PathBuf {
        PathBuf::from(DIR_AGENTS_SKILLS)
            .join(name)
            .join(CODEX_OPENAI_POLICY_RELATIVE_PATH)
    }

    fn policy_command(name: &str, metadata: serde_json::Value) -> Resource {
        Resource::Command(ResourceData {
            name: name.to_string(),
            plugin: "test-plugin".to_string(),
            content: String::new(),
            metadata,
            source_path: PathBuf::from(format!("src/{name}.md")),
        })
    }

    fn policy_skill(name: &str, metadata: serde_json::Value, extras: Vec<ExtraFile>) -> Resource {
        Resource::Skill(SkillData {
            base: ResourceData {
                name: name.to_string(),
                plugin: "test-plugin".to_string(),
                content: String::new(),
                metadata,
                source_path: PathBuf::from(format!("src/{name}")),
            },
            extras,
        })
    }

    #[test]
    fn test_codex_command_transformation() {
        let transformer = CodexTransformer;
        let resource = Resource::Command(ResourceData {
            name: "test-cmd".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Hello Codex".to_string(),
            metadata: json!({
                "description": "A test command"
            }),
            source_path: PathBuf::from("src/test.md"),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(
            result.path,
            PathBuf::from(DIR_AGENTS_SKILLS).join("test-cmd").join(SKILL_MD)
        );
        assert!(result.content.contains("description: A test command"));
        assert!(result.content.contains("Hello Codex"));
    }

    #[test]
    fn test_codex_skill_transformation() {
        let transformer = CodexTransformer;
        let resource = Resource::Skill(SkillData {
            base: ResourceData {
                name: "test-skill".to_string(),
                plugin: "test-plugin".to_string(),
                content: "Skill Content".to_string(),
                metadata: json!({
                    "description": "Skill description"
                }),
                source_path: PathBuf::from("src/test-skill"),
            },
            extras: Vec::new(),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(
            result.path,
            PathBuf::from(DIR_AGENTS_SKILLS).join("test-skill").join(SKILL_MD)
        );
        assert!(result.content.contains("description: Skill description"));
        assert!(result.content.contains("Skill Content"));
    }

    #[test]
    fn test_codex_agent_transformation() {
        let transformer = CodexTransformer;
        let resource = Resource::Agent(ResourceData {
            name: "test-agent".to_string(),
            plugin: "test-plugin".to_string(),
            content: "You are a codex agent.".to_string(),
            metadata: json!({
                "model": "codex-latest",
                "temperature": 0.3,
                "description": "Agent description"
            }),
            source_path: PathBuf::from("src/test.md"),
        });

        let result = transformer.transform(&resource).unwrap();
        assert_eq!(
            result.path,
            PathBuf::from(DIR_AGENTS).join(format!("test-agent{}", EXT_TOML))
        );

        let toml_val: Table = toml::from_str(&result.content).unwrap();
        assert_eq!(toml_val.get("model").unwrap().as_str().unwrap(), "codex-latest");
        assert_eq!(toml_val.get("temperature").unwrap().as_float().unwrap(), 0.3);
        assert!(toml_val.get("description").is_none());
        assert_eq!(
            toml_val.get("developer_instructions").unwrap().as_str().unwrap(),
            "You are a codex agent."
        );
    }

    #[test]
    fn test_codex_agent_multiline_transformation() {
        let transformer = CodexTransformer;
        let resource = Resource::Agent(ResourceData {
            name: "test-agent".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Line 1\nLine 2".to_string(),
            metadata: json!({}),
            source_path: PathBuf::from("src/test.md"),
        });

        let result = transformer.transform(&resource).unwrap();
        let toml_val: Table = toml::from_str(&result.content).unwrap();
        assert_eq!(
            toml_val.get("developer_instructions").unwrap().as_str().unwrap(),
            "Line 1\nLine 2\n"
        );
    }

    #[test]
    fn test_codex_detransform_agent() {
        let transformer = CodexTransformer;
        let input = r#"model = "codex-001"
developer_instructions = "Agent Logic"
"#;

        let result = transformer
            .detransform(ResourceType::Agent, "test", input, std::path::Path::new(""))
            .unwrap();

        assert_eq!(result.content, "Agent Logic");
        assert_eq!(result.metadata["model"], "codex-001");
    }

    #[test]
    fn test_codex_post_transform() {
        let transformer = CodexTransformer;
        let r1 = Resource::Agent(ResourceData {
            name: "test-agent-1".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Agent 1".to_string(),
            metadata: json!({
                "description": "Desc 1"
            }),
            source_path: PathBuf::from(""),
        });
        let r2 = Resource::Agent(ResourceData {
            name: "test-agent-2".to_string(),
            plugin: "test-plugin".to_string(),
            content: "Agent 2".to_string(),
            metadata: json!({}),
            source_path: PathBuf::from(""),
        });

        let resources = vec![&r1, &r2];
        let result = transformer.post_transform(&resources).unwrap();

        assert_eq!(result.len(), 1);
        let config_file = &result[0];
        assert_eq!(config_file.path, PathBuf::from(CODEX_CONFIG_FILE_NAME));

        let root: Table = toml::from_str(&config_file.content).unwrap();
        let agents = root.get("agents").unwrap().as_table().unwrap();

        let a1 = agents.get("test-agent-1").unwrap().as_table().unwrap();
        assert_eq!(a1.get("description").unwrap().as_str().unwrap(), "Desc 1");
        assert_eq!(
            a1.get("config_file").unwrap().as_str().unwrap(),
            "agents/test-agent-1.toml"
        );

        let a2 = agents.get("test-agent-2").unwrap().as_table().unwrap();
        assert_eq!(a2.get("description").unwrap().as_str().unwrap(), "");
        assert_eq!(
            a2.get("config_file").unwrap().as_str().unwrap(),
            "agents/test-agent-2.toml"
        );
    }

    #[test]
    fn test_codex_post_transform_generates_openai_policy_for_command() {
        let transformer = CodexTransformer;
        let command = policy_command(
            "policy-cmd",
            json!({
                "disable-model-invocation": true
            }),
        );

        let result = transformer.post_transform(&[&command]).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path, policy_path("policy-cmd"));
        assert_eq!(result[0].content, OPENAI_POLICY_CONTENT);
    }

    #[test]
    fn test_codex_post_transform_generates_openai_policy_for_skill_without_source_extra() {
        let transformer = CodexTransformer;
        let skill = policy_skill(
            "policy-skill",
            json!({
                "disable-model-invocation": true
            }),
            Vec::new(),
        );

        let result = transformer.post_transform(&[&skill]).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path, policy_path("policy-skill"));
        assert_eq!(result[0].content, OPENAI_POLICY_CONTENT);
    }

    #[test]
    fn test_codex_post_transform_skips_generated_policy_for_skill_with_source_extra() {
        let transformer = CodexTransformer;
        let skill_root = PathBuf::from("src/policy-skill");
        let skill = Resource::Skill(SkillData {
            base: ResourceData {
                name: "policy-skill".to_string(),
                plugin: "test-plugin".to_string(),
                content: String::new(),
                metadata: json!({
                    "disable-model-invocation": true
                }),
                source_path: skill_root.clone(),
            },
            extras: vec![ExtraFile {
                source: skill_root.join(CODEX_OPENAI_POLICY_RELATIVE_PATH),
                target: policy_path("policy-skill"),
            }],
        });

        let result = transformer.post_transform(&[&skill]).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn test_codex_post_transform_does_not_generate_policy_for_non_true_metadata() {
        let transformer = CodexTransformer;
        let cases = [
            json!({}),
            json!({ "disable-model-invocation": false }),
            json!({ "disable-model-invocation": "true" }),
            json!({ "disable-model-invocation": "false" }),
            json!({ "disable-model-invocation": null }),
        ];

        for (index, metadata) in cases.into_iter().enumerate() {
            let command = policy_command(&format!("policy-cmd-{index}"), metadata);
            let result = transformer.post_transform(&[&command]).unwrap();
            assert!(result.is_empty());
        }
    }

    #[test]
    fn test_codex_post_transform_keeps_agent_registry_with_policy_files() {
        let transformer = CodexTransformer;
        let agent = Resource::Agent(ResourceData {
            name: "test-agent".to_string(),
            plugin: "test-plugin".to_string(),
            content: String::new(),
            metadata: json!({
                "description": "Agent description"
            }),
            source_path: PathBuf::new(),
        });
        let command = policy_command(
            "policy-cmd",
            json!({
                "disable-model-invocation": true
            }),
        );

        let result = transformer.post_transform(&[&agent, &command]).unwrap();

        assert_eq!(result.len(), 2);
        assert!(
            result
                .iter()
                .any(|file| file.path.as_path() == Path::new(CODEX_CONFIG_FILE_NAME))
        );
        assert!(result.iter().any(|file| file.path == policy_path("policy-cmd")));
    }

    #[test]
    fn test_codex_generated_extra_ignore_paths_for_generated_skill_policy() {
        let transformer = CodexTransformer;
        let skill = policy_skill(
            "policy-skill",
            json!({
                "disable-model-invocation": true
            }),
            Vec::new(),
        );

        let ignored_paths = transformer.generated_extra_ignore_paths(&skill);

        assert_eq!(ignored_paths, vec![PathBuf::from(CODEX_OPENAI_POLICY_RELATIVE_PATH)]);
    }

    #[test]
    fn test_codex_generated_extra_ignore_paths_empty_for_source_policy() {
        let transformer = CodexTransformer;
        let skill_root = PathBuf::from("src/policy-skill");
        let skill = Resource::Skill(SkillData {
            base: ResourceData {
                name: "policy-skill".to_string(),
                plugin: "test-plugin".to_string(),
                content: String::new(),
                metadata: json!({
                    "disable-model-invocation": true
                }),
                source_path: skill_root.clone(),
            },
            extras: vec![ExtraFile {
                source: skill_root.join(CODEX_OPENAI_POLICY_RELATIVE_PATH),
                target: policy_path("policy-skill"),
            }],
        });

        let ignored_paths = transformer.generated_extra_ignore_paths(&skill);

        assert!(ignored_paths.is_empty());
    }
}
