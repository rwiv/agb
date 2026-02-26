use crate::core::{
    BuildTarget, DIR_AGENTS, DIR_COMMANDS, DIR_SKILLS, EXT_YAML, EXT_YML, ExtraFile, Resource, ResourceData, SkillData,
};
use crate::loader::{ScannedPaths, ScannedResource};
use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

/// 리소스를 파싱하고 조립하는 객체입니다.
pub struct ResourceParser {
    pub target: BuildTarget,
}

/// 파싱 과정에서 공통으로 사용되는 정보를 담는 내부 컨텍스트입니다.
struct ParseContext<'a> {
    plugin: &'a str,
    name: &'a str,
    md: Option<PathBuf>,
    metadata: Option<PathBuf>,
}

impl ResourceParser {
    pub fn new(target: BuildTarget) -> Self {
        Self { target }
    }

    /// 스캔된 리소스 정보로부터 Resource 객체를 생성합니다.
    pub fn parse_resource(&self, scanned: ScannedResource) -> Result<Resource> {
        let plugin = scanned.plugin;
        let name = scanned.name;

        // 리소스 타입에 따른 정보 추출
        let (r_type, md, metadata, extras) = match scanned.paths {
            ScannedPaths::Command { md, metadata } => (DIR_COMMANDS, md, metadata, None),
            ScannedPaths::Agent { md, metadata } => (DIR_AGENTS, md, metadata, None),
            ScannedPaths::Skill { md, metadata, extras } => (DIR_SKILLS, md, metadata, Some(extras)),
        };

        let ctx = ParseContext {
            plugin: &plugin,
            name: &name,
            md,
            metadata,
        };

        let data = self.parse_common(r_type, ctx)?;

        match r_type {
            DIR_COMMANDS => Ok(Resource::Command(data)),
            DIR_AGENTS => Ok(Resource::Agent(data)),
            DIR_SKILLS => {
                let skill_extras = extras
                    .unwrap_or_default()
                    .into_iter()
                    .map(|source| {
                        let file_name = source.file_name().unwrap().to_os_string();
                        let target = PathBuf::from(DIR_SKILLS).join(&name).join(file_name);
                        ExtraFile { source, target }
                    })
                    .collect();

                Ok(Resource::Skill(SkillData {
                    base: data,
                    extras: skill_extras,
                }))
            }
            _ => unreachable!("Unknown resource type: {}", r_type),
        }
    }

    /// 공통 데이터 파싱 로직 (Markdown + Metadata)
    fn parse_common(&self, r_type: &str, ctx: ParseContext) -> Result<ResourceData> {
        // 1. Markdown 본문 및 Frontmatter 추출
        let (mut fm_metadata, pure_content) = if let Some(ref p) = ctx.md {
            let raw_content =
                fs::read_to_string(p).with_context(|| format!("Failed to read markdown content: {:?}", p))?;
            crate::utils::yaml::extract_frontmatter(&raw_content)
        } else {
            anyhow::bail!(
                "Markdown file is missing for resource '{}' in plugin '{}' ({})",
                ctx.name,
                ctx.plugin,
                r_type
            );
        };

        // 2. 외부 메타데이터 파일 파싱 및 병합
        if let Some(ref p) = ctx.metadata {
            let ext_metadata = self.parse_metadata(p, r_type, &ctx)?;
            self.merge_metadata(&mut fm_metadata, &ext_metadata)
                .with_context(|| format!("Failed to merge metadata for resource: {}/{}", r_type, ctx.name))?;
        }

        Ok(ResourceData {
            name: ctx.name.to_string(),
            plugin: ctx.plugin.to_string(),
            content: pure_content,
            metadata: fm_metadata,
        })
    }

    /// 파일 경로로부터 메타데이터를 파싱하여 serde_json::Value로 반환합니다.
    fn parse_metadata(&self, path: &Path, r_type: &str, ctx: &ParseContext) -> Result<Value> {
        let meta_str = fs::read_to_string(path).with_context(|| format!("Failed to read metadata file: {:?}", path))?;

        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();

        if extension == &EXT_YAML[1..] || extension == &EXT_YML[1..] {
            serde_yaml::from_str(&meta_str)
                .with_context(|| format!("Failed to parse YAML for resource: {}/{}", r_type, ctx.name))
        } else {
            anyhow::bail!(
                "Unsupported metadata extension '{}' for resource: {}/{}",
                extension,
                r_type,
                ctx.name
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
        let scanned = ScannedResource {
            plugin: "p1".to_string(),
            name: "foo".to_string(),
            paths: ScannedPaths::Command {
                md: Some(md_path),
                metadata: Some(yaml_path),
            },
        };

        let res = parser.parse_resource(scanned)?;
        if let Resource::Command(d) = res {
            assert_eq!(d.name, "foo");
            assert_eq!(d.content, "# Content");
            assert_eq!(d.metadata["model"], "gemini-model");
        } else {
            panic!("Expected Command resource");
        }
        Ok(())
    }

    #[test]
    fn test_parse_skill_with_extras() -> Result<()> {
        let dir = tempdir()?;
        let skill_dir = dir.path().join("p1/skills/my-skill");
        fs::create_dir_all(&skill_dir)?;

        let md_path = skill_dir.join("SKILL.md");
        let extra_path = skill_dir.join("logic.py");
        fs::write(&md_path, "# Skill")?;
        fs::write(&extra_path, "print('hello')")?;

        let parser = ResourceParser::new(BuildTarget::GeminiCli);
        let scanned = ScannedResource {
            plugin: "p1".to_string(),
            name: "my-skill".to_string(),
            paths: ScannedPaths::Skill {
                md: Some(md_path),
                metadata: None,
                extras: vec![extra_path],
            },
        };

        let res = parser.parse_resource(scanned)?;
        if let Resource::Skill(s) = res {
            assert_eq!(s.base.name, "my-skill");
            assert_eq!(s.extras.len(), 1);
            assert_eq!(s.extras[0].target.to_str().unwrap(), "skills/my-skill/logic.py");
        } else {
            panic!("Expected Skill resource");
        }
        Ok(())
    }
}
