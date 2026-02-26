use crate::core::{
    BuildTarget, DIR_AGENTS, DIR_COMMANDS, DIR_SKILLS, EXT_YAML, EXT_YML, Resource, ResourceData, ResourceKey,
    ResourcePaths,
};
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
        self.merge_metadata(&mut fm_metadata, &ext_metadata)
            .with_context(|| format!("Failed to merge metadata for resource: {}/{}", r_type, resource_name))?;

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

        if r_type == DIR_COMMANDS {
            Ok(Resource::Command(data))
        } else if r_type == DIR_AGENTS {
            Ok(Resource::Agent(data))
        } else if r_type == DIR_SKILLS {
            Ok(Resource::Skill(data))
        } else {
            anyhow::bail!("Unknown resource type: {}", r_type)
        }
    }

    /// 파일 경로로부터 메타데이터를 파싱하여 serde_json::Value로 반환합니다.
    fn parse_metadata(&self, path: &Path, resource_type: &str, resource_name: &str) -> Result<Value> {
        let meta_str = fs::read_to_string(path).with_context(|| format!("Failed to read metadata file: {:?}", path))?;

        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();

        if extension == &EXT_YAML[1..] || extension == &EXT_YML[1..] {
            serde_yaml::from_str(&meta_str)
                .with_context(|| format!("Failed to parse YAML for resource: {}/{}", resource_type, resource_name))
        } else {
            anyhow::bail!(
                "Unsupported metadata extension '{}' for resource: {}/{}",
                extension,
                resource_type,
                resource_name
            )
        }
    }

    /// 두 개의 메타데이터 객체를 타겟 규칙에 따라 병합합니다.
    fn merge_metadata(&self, base: &mut Value, external: &Value) -> Result<()> {
        if !base.is_object() {
            *base = json!({});
        }

        let base_obj = base.as_object_mut().unwrap();
        let reserved_keys = BuildTarget::all_reserved_keys();

        if let Some(ext_obj) = external.as_object() {
            // 1. 외부 파일의 최상위 키 검증 (예약어만 허용)
            for k in ext_obj.keys() {
                if !reserved_keys.contains(&k.as_str()) {
                    anyhow::bail!(
                        "External metadata contains unauthorized top-level field: '{}'. \
                         Only target reserved keys ({:?}) are allowed.",
                        k,
                        reserved_keys
                    );
                }
            }

            // 2. 타겟 전용 필드들로 최종 오버라이트 (Shallow merge)
            let target_key = self.target.reserved_key();
            if let Some(target_section) = ext_obj.get(target_key).and_then(|v| v.as_object()) {
                for (k, v) in target_section {
                    base_obj.insert(k.clone(), v.clone());
                }
            }
        }

        // 3. 결과물에서 모든 타겟 섹션 예약어 키들 제거
        for key in reserved_keys {
            base_obj.remove(*key);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_merge_metadata_target_only_override() -> Result<()> {
        let parser = ResourceParser::new(BuildTarget::GeminiCli);
        let mut base = json!({
            "name": "my-agent",
            "model": "default-model",
            "temperature": 0.5
        });
        // Valid external metadata: only target keys
        let external = json!({
            "gemini-cli": {
                "model": "gemini-3.0-pro",
                "temperature": 0.2
            },
            "claude-code": {
                "model": "claude-3-opus"
            }
        });

        parser.merge_metadata(&mut base, &external)?;

        // Common fields like 'name' should NOT be overwritten from external (because they shouldn't exist there)
        assert_eq!(base["name"], "my-agent");
        assert_eq!(base["model"], "gemini-3.0-pro");
        assert_eq!(base["temperature"], 0.2);
        assert!(base.get("gemini-cli").is_none());
        assert!(base.get("claude-code").is_none());
        Ok(())
    }

    #[test]
    fn test_merge_metadata_validation_fail() {
        let parser = ResourceParser::new(BuildTarget::GeminiCli);
        let mut base = json!({"name": "foo"});
        // Invalid external metadata: contains 'name' at top-level
        let external = json!({
            "name": "illegal-override",
            "gemini-cli": { "model": "bar" }
        });

        let result = parser.merge_metadata(&mut base, &external);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unauthorized top-level field: 'name'"));
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
            "gemini-cli:
  model: gemini-model",
        )?;

        let parser = ResourceParser::new(BuildTarget::GeminiCli);
        let key = ResourceKey {
            plugin: "p1".to_string(),
            r_type: DIR_COMMANDS.to_string(),
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
    fn test_resource_parser_parse_resource_command_no_ext_metadata() -> Result<()> {
        let dir = tempdir()?;
        let md_path = dir.path().join("foo.md");
        fs::write(&md_path, "# Content")?;

        let parser = ResourceParser::new(BuildTarget::GeminiCli);
        let key = ResourceKey {
            plugin: "p1".to_string(),
            r_type: DIR_COMMANDS.to_string(),
            name: "foo".to_string(),
        };
        let paths = ResourcePaths {
            md: Some(md_path),
            metadata: None,
        };

        let res = parser.parse_resource(key, paths)?;
        if let Resource::Command(d) = res {
            assert_eq!(d.name, "foo");
            assert_eq!(d.plugin, "p1");
            assert_eq!(d.content, "# Content");
        } else {
            panic!("Expected Command resource");
        }
        Ok(())
    }

    #[test]
    fn test_resource_parser_validation_error_on_general_field() -> Result<()> {
        let dir = tempdir()?;
        let md_path = dir.path().join("foo.md");
        let yaml_path = dir.path().join("foo.yaml");
        fs::write(&md_path, "# Content")?;
        fs::write(&yaml_path, "key: val")?; // General field 'key' is unauthorized

        let parser = ResourceParser::new(BuildTarget::GeminiCli);
        let key = ResourceKey {
            plugin: "p1".to_string(),
            r_type: DIR_COMMANDS.to_string(),
            name: "foo".to_string(),
        };
        let paths = ResourcePaths {
            md: Some(md_path),
            metadata: Some(yaml_path),
        };

        let res = parser.parse_resource(key, paths);
        assert!(res.is_err());
        let err_msg = format!("{:#}", res.unwrap_err());
        assert!(err_msg.contains("unauthorized top-level field: 'key'"));
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
        let value = parser.parse_metadata(&yaml_path, DIR_COMMANDS, "test")?;

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
        let result = parser.parse_metadata(&txt_path, DIR_COMMANDS, "test");

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
