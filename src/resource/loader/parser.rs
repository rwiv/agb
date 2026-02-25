use crate::resource::{BuildTarget, Resource, ResourceData, ResourceKey, ResourcePaths};
use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

/// 리소스를 파싱하고 조립하는 객체입니다.
pub struct ResourceParser {
    pub target: BuildTarget,
}

impl ResourceParser {
    pub fn new(target: BuildTarget) -> Self {
        Self { target }
    }

    /// 두 개의 메타데이터 객체를 타겟 규칙에 따라 병합합니다.
    fn merge_metadata(&self, base: &mut Value, external: &Value, target: &BuildTarget) {
        if !base.is_object() {
            *base = json!({});
        }

        let base_obj = base.as_object_mut().unwrap();

        if let Some(ext_obj) = external.as_object() {
            // 1. 외부 파일의 일반 필드들을 base에 덮어씀 (Shallow merge)
            for (k, v) in ext_obj {
                if k != "gemini" && k != "claude" && k != "opencode" {
                    base_obj.insert(k.clone(), v.clone());
                }
            }

            // 2. 타겟 섹션 키 결정
            let target_key = match target {
                BuildTarget::GeminiCli => "gemini",
                BuildTarget::ClaudeCode => "claude",
                BuildTarget::OpenCode => "opencode",
            };

            // 3. 타겟 전용 필드들로 최종 오버라이트
            if let Some(target_section) = ext_obj.get(target_key).and_then(|v| v.as_object()) {
                for (k, v) in target_section {
                    base_obj.insert(k.clone(), v.clone());
                }
            }
        }

        // 4. 결과물에서 타겟 섹션 예약어 키들 제거
        base_obj.remove("gemini");
        base_obj.remove("claude");
        base_obj.remove("opencode");
    }

    /// 파일 경로로부터 메타데이터를 파싱하여 serde_json::Value로 반환합니다.
    pub fn parse_metadata(&self, path: &Path, resource_type: &str, resource_name: &str) -> Result<Value> {
        let meta_str = fs::read_to_string(path).with_context(|| format!("Failed to read metadata file: {:?}", path))?;

        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();

        match extension {
            "yaml" | "yml" => serde_yaml::from_str(&meta_str)
                .with_context(|| format!("Failed to parse YAML for resource: {}/{}", resource_type, resource_name)),
            _ => anyhow::bail!(
                "Unsupported metadata extension '{}' for resource: {}/{}",
                extension,
                resource_type,
                resource_name
            ),
        }
    }

    /// 그룹화된 파일 경로들로부터 Resource 객체를 생성합니다.
    pub fn parse_resource(&self, key: ResourceKey, paths: ResourcePaths) -> Result<Resource> {
        let ResourceKey {
            plugin,
            r_type,
            name: mut resource_name,
        } = key;
        let ResourcePaths { md, metadata } = paths;

        // 1. Markdown 본문 및 Frontmatter 추출
        let (mut fm_metadata, pure_content) = if let Some(p) = md {
            let raw_content =
                fs::read_to_string(&p).with_context(|| format!("Failed to read markdown content: {:?}", p))?;
            crate::utils::yaml::extract_frontmatter(&raw_content)
        } else {
            (json!({}), String::new())
        };

        // 2. 외부 메타데이터 파일 파싱
        let ext_metadata = if let Some(p) = metadata {
            self.parse_metadata(&p, &r_type, &resource_name)?
        } else {
            json!({})
        };

        // 3. 타겟 규칙에 따른 병합 (FM + External)
        self.merge_metadata(&mut fm_metadata, &ext_metadata, &self.target);

        // 4. 명시된 이름이 있다면 리소스 이름으로 사용
        if let Some(explicit_name) = fm_metadata.get("name").and_then(|v| v.as_str()) {
            resource_name = explicit_name.to_string();
        }

        let data = ResourceData {
            name: resource_name,
            plugin,
            content: pure_content,
            metadata: fm_metadata,
        };

        match r_type.as_str() {
            "commands" => Ok(Resource::Command(data)),
            "agents" => Ok(Resource::Agent(data)),
            "skills" => Ok(Resource::Skill(data)),
            _ => anyhow::bail!("Unknown resource type: {}", r_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_merge_metadata_target_aware() {
        let parser = ResourceParser::new(BuildTarget::GeminiCli);
        let mut base = json!({
            "name": "my-agent",
            "model": "default-model",
            "temperature": 0.5
        });
        let external = json!({
            "name": "overwritten-name",
            "gemini": {
                "model": "gemini-3.0-pro",
                "temperature": 0.2
            },
            "claude": {
                "model": "claude-3-opus"
            }
        });

        parser.merge_metadata(&mut base, &external, &BuildTarget::GeminiCli);

        assert_eq!(base["name"], "overwritten-name");
        assert_eq!(base["model"], "gemini-3.0-pro");
        assert_eq!(base["temperature"], 0.2);
        assert!(base.get("gemini").is_none());
        assert!(base.get("claude").is_none());
    }

    #[test]
    fn test_parse_resource_with_frontmatter_and_external() -> Result<()> {
        let dir = tempdir()?;
        let md_path = dir.path().join("foo.md");
        let yaml_path = dir.path().join("foo.yaml");

        fs::write(
            &md_path,
            "---
name: fm-name
model: fm-model
---
# Content",
        )?;
        fs::write(
            &yaml_path,
            "gemini:
  model: gemini-model",
        )?;

        let parser = ResourceParser::new(BuildTarget::GeminiCli);
        let key = ResourceKey {
            plugin: "p1".to_string(),
            r_type: "commands".to_string(),
            name: "foo".to_string(),
        };
        let paths = ResourcePaths {
            md: Some(md_path),
            metadata: Some(yaml_path),
        };

        let res = parser.parse_resource(key, paths)?;
        if let Resource::Command(d) = res {
            assert_eq!(d.name, "fm-name");
            assert_eq!(d.content, "# Content");
            assert_eq!(d.metadata["model"], "gemini-model");
        } else {
            panic!("Expected Command resource");
        }
        Ok(())
    }

    #[test]
    fn test_resource_parser_parse_resource_command() -> Result<()> {
        let dir = tempdir()?;
        let md_path = dir.path().join("foo.md");
        let yaml_path = dir.path().join("foo.yaml");
        fs::write(&md_path, "# Content")?;
        fs::write(&yaml_path, "key: val")?;

        let parser = ResourceParser::new(BuildTarget::GeminiCli);
        let key = ResourceKey {
            plugin: "p1".to_string(),
            r_type: "commands".to_string(),
            name: "foo".to_string(),
        };
        let paths = ResourcePaths {
            md: Some(md_path),
            metadata: Some(yaml_path),
        };

        let res = parser.parse_resource(key, paths)?;
        if let Resource::Command(d) = res {
            assert_eq!(d.name, "foo");
            assert_eq!(d.plugin, "p1");
            assert_eq!(d.content, "# Content");
            assert_eq!(d.metadata["key"], "val");
        } else {
            panic!("Expected Command resource");
        }
        Ok(())
    }

    #[test]
    fn test_metadata_parser_parse_yaml() -> Result<()> {
        let dir = tempdir()?;
        let yaml_path = dir.path().join("test.yaml");
        fs::write(
            &yaml_path,
            "key: val
num: 123",
        )?;

        let parser = ResourceParser::new(BuildTarget::GeminiCli);
        let value = parser.parse_metadata(&yaml_path, "commands", "test")?;

        assert_eq!(value["key"], "val");
        assert_eq!(value["num"], 123);
        Ok(())
    }

    #[test]
    fn test_metadata_parser_unsupported_extension() -> Result<()> {
        let dir = tempdir()?;
        let txt_path = dir.path().join("test.txt");
        fs::write(&txt_path, "content")?;

        let parser = ResourceParser::new(BuildTarget::GeminiCli);
        let result = parser.parse_metadata(&txt_path, "commands", "test");

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Unsupported metadata extension 'txt'")
        );
        Ok(())
    }
}
