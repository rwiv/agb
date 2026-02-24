pub mod filter;
pub mod parser;
pub mod resolver;

use crate::resource::{Resource, ResourceData};
use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use self::filter::FileFilter;
use self::parser::MetadataParser;
use self::resolver::{ResourceKey, ResourcePathResolver, ResourcePaths};

/// 플러그인 디렉터리를 탐색하고 리소스를 로드하는 객체입니다.
pub struct ResourceLoader {
    root: PathBuf,
    filter: FileFilter,
    resolver: ResourcePathResolver,
    parser: MetadataParser,
}

impl ResourceLoader {
    /// 새로운 ResourceLoader 인스턴스를 생성합니다.
    pub fn new<P: AsRef<Path>>(root: P, exclude_patterns: &[String]) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        if !root.exists() {
            anyhow::bail!("Plugins directory not found: {:?}", root);
        }

        let filter = FileFilter::new(exclude_patterns)?;
        let resolver = ResourcePathResolver::new();
        let parser = MetadataParser::new();

        Ok(Self {
            root,
            filter,
            resolver,
            parser,
        })
    }

    /// 리소스를 로드합니다.
    pub fn load(&self) -> Result<Vec<Resource>> {
        let files = self.scan()?;
        let groups = self.resolver.resolve(&self.root, files)?;

        groups
            .into_iter()
            .map(|(key, paths)| self.parse_resource(key, paths))
            .collect()
    }

    /// 플러그인 디렉터리를 스캔하여 유효한 파일 경로 목록을 반환합니다.
    fn scan(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.root).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if self.filter.is_valid(&self.root, path)? {
                files.push(path.to_path_buf());
            }
        }

        Ok(files)
    }

    /// 그룹화된 파일 경로들로부터 Resource 객체를 생성합니다.
    fn parse_resource(&self, key: ResourceKey, paths: ResourcePaths) -> Result<Resource> {
        let ResourceKey { plugin, r_type, name } = key;
        let ResourcePaths { md, metadata } = paths;

        let content = if let Some(p) = md {
            fs::read_to_string(p)?
        } else {
            String::new()
        };

        let metadata_val = if let Some(p) = metadata {
            self.parser.parse(&p, &r_type, &name)?
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
    fn test_resource_loader_load_integration() -> Result<()> {
        let dir = tempdir()?;
        let plugins_path = dir.path().join("plugins");

        // 샘플 구조 생성
        let cmd_dir = plugins_path.join("plugin_a/commands");
        let skill_dir = plugins_path.join("plugin_b/skills/my_skill");
        fs::create_dir_all(&cmd_dir)?;
        fs::create_dir_all(&skill_dir)?;

        // Command: md + json
        fs::write(cmd_dir.join("foo.md"), "# Foo Content")?;
        fs::write(cmd_dir.join("foo.json"), "{\"key\": \"val\"}")?;
        // Exclude 대상
        fs::write(cmd_dir.join("test.tmp"), "temp")?;

        // Skill: [skill_name].json + md
        fs::write(skill_dir.join("my_skill.json"), "{\"desc\": \"skill\"}")?;
        fs::write(skill_dir.join("logic.md"), "prompt")?;

        let loader = ResourceLoader::new(&plugins_path, &["*.tmp".to_string()])?;
        let resources = loader.load()?;

        assert_eq!(resources.len(), 2);

        let mut found_foo = false;
        let mut found_skill = false;

        for res in resources {
            match res {
                Resource::Command(d) if d.name == "foo" => {
                    assert_eq!(d.plugin, "plugin_a");
                    assert_eq!(d.content, "# Foo Content");
                    assert_eq!(d.metadata["key"], "val");
                    found_foo = true;
                }
                Resource::Skill(d) if d.name == "my_skill" => {
                    assert_eq!(d.plugin, "plugin_b");
                    assert_eq!(d.metadata["desc"], "skill");
                    assert!(d.content.contains("prompt"));
                    found_skill = true;
                }
                _ => {}
            }
        }

        assert!(found_foo);
        assert!(found_skill);

        Ok(())
    }
}
