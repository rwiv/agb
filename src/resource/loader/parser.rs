use crate::resource::{Resource, ResourceData, ResourceKey, ResourcePaths};
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// 리소스를 파싱하고 조립하는 객체입니다.
pub struct ResourceParser;

impl ResourceParser {
    pub fn new() -> Self {
        Self
    }

    /// 파일 경로로부터 메타데이터를 파싱하여 serde_json::Value로 반환합니다.
    pub fn parse_metadata(&self, path: &Path, resource_type: &str, resource_name: &str) -> Result<Value> {
        let meta_str = fs::read_to_string(path).with_context(|| format!("Failed to read metadata file: {:?}", path))?;

        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();

        match extension {
            "json" => serde_json::from_str(&meta_str)
                .with_context(|| format!("Failed to parse JSON for resource: {}/{}", resource_type, resource_name)),
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
        let ResourceKey { plugin, r_type, name } = key;
        let ResourcePaths { md, metadata } = paths;

        let content = if let Some(p) = md {
            fs::read_to_string(&p).with_context(|| format!("Failed to read markdown content: {:?}", p))?
        } else {
            String::new()
        };

        let metadata_val = if let Some(p) = metadata {
            self.parse_metadata(&p, &r_type, &name)?
        } else {
            Value::Null
        };

        let data = ResourceData {
            name: name.clone(),
            plugin,
            content,
            metadata: metadata_val,
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
    fn test_resource_parser_parse_resource_command() -> Result<()> {
        let dir = tempdir()?;
        let md_path = dir.path().join("foo.md");
        let json_path = dir.path().join("foo.json");
        fs::write(&md_path, "# Content")?;
        fs::write(&json_path, r#"{"key": "val"}"#)?;

        let parser = ResourceParser::new();
        let key = ResourceKey {
            plugin: "p1".to_string(),
            r_type: "commands".to_string(),
            name: "foo".to_string(),
        };
        let paths = ResourcePaths {
            md: Some(md_path),
            metadata: Some(json_path),
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
    fn test_metadata_parser_parse_json() -> Result<()> {
        let dir = tempdir()?;
        let json_path = dir.path().join("test.json");
        fs::write(&json_path, r#"{"key": "val"}"#)?;

        let parser = ResourceParser::new();
        let value = parser.parse_metadata(&json_path, "commands", "test")?;

        assert_eq!(value["key"], "val");
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

        let parser = ResourceParser::new();
        let value = parser.parse_metadata(&yaml_path, "commands", "test")?;

        assert_eq!(value["key"], "val");
        assert_eq!(value["num"], 123);
        Ok(())
    }

    #[test]
    fn test_metadata_parser_invalid_json() -> Result<()> {
        let dir = tempdir()?;
        let json_path = dir.path().join("bad.json");
        fs::write(&json_path, "{ invalid }")?;

        let parser = ResourceParser::new();
        let result = parser.parse_metadata(&json_path, "commands", "bad");

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Failed to parse JSON"));
        assert!(err_msg.contains("commands/bad"));
        Ok(())
    }

    #[test]
    fn test_metadata_parser_unsupported_extension() -> Result<()> {
        let dir = tempdir()?;
        let txt_path = dir.path().join("test.txt");
        fs::write(&txt_path, "content")?;

        let parser = ResourceParser::new();
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
