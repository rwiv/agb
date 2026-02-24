use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// 리소스 메타데이터를 파싱하는 객체입니다.
pub struct MetadataParser;

impl MetadataParser {
    pub fn new() -> Self {
        Self
    }

    /// 파일 경로로부터 메타데이터를 파싱하여 serde_json::Value로 반환합니다.
    pub fn parse(&self, path: &Path, resource_type: &str, resource_name: &str) -> Result<Value> {
        let meta_str = fs::read_to_string(path)
            .with_context(|| format!("Failed to read metadata file: {:?}", path))?;

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default();

        match extension {
            "json" => {
                serde_json::from_str(&meta_str)
                    .with_context(|| format!("Failed to parse JSON for resource: {}/{}", resource_type, resource_name))
            }
            "yaml" | "yml" => {
                serde_yaml::from_str(&meta_str)
                    .with_context(|| format!("Failed to parse YAML for resource: {}/{}", resource_type, resource_name))
            }
            _ => anyhow::bail!(
                "Unsupported metadata extension '{}' for resource: {}/{}",
                extension,
                resource_type,
                resource_name
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_metadata_parser_parse_json() -> Result<()> {
        let dir = tempdir()?;
        let json_path = dir.path().join("test.json");
        fs::write(&json_path, r#"{"key": "val"}"#)?;

        let parser = MetadataParser::new();
        let value = parser.parse(&json_path, "commands", "test")?;

        assert_eq!(value["key"], "val");
        Ok(())
    }

    #[test]
    fn test_metadata_parser_parse_yaml() -> Result<()> {
        let dir = tempdir()?;
        let yaml_path = dir.path().join("test.yaml");
        fs::write(&yaml_path, "key: val
num: 123")?;

        let parser = MetadataParser::new();
        let value = parser.parse(&yaml_path, "commands", "test")?;

        assert_eq!(value["key"], "val");
        assert_eq!(value["num"], 123);
        Ok(())
    }

    #[test]
    fn test_metadata_parser_invalid_json() -> Result<()> {
        let dir = tempdir()?;
        let json_path = dir.path().join("bad.json");
        fs::write(&json_path, "{ invalid }")?;

        let parser = MetadataParser::new();
        let result = parser.parse(&json_path, "commands", "bad");

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

        let parser = MetadataParser::new();
        let result = parser.parse(&txt_path, "commands", "test");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unsupported metadata extension 'txt'"));
        Ok(())
    }
}
