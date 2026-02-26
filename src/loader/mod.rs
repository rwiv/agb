pub mod filter;
pub mod parser;
pub mod resolver;

use crate::core::{BuildTarget, Resource};
use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use self::filter::FileFilter;
use self::parser::ResourceParser;
use self::resolver::ResourcePathResolver;

/// 플러그인 디렉터리를 탐색하고 리소스를 로드하는 객체입니다.
pub struct ResourceLoader {
    root: PathBuf,
    filter: FileFilter,
    resolver: ResourcePathResolver,
    parser: ResourceParser,
}

impl ResourceLoader {
    /// 새로운 ResourceLoader 인스턴스를 생성합니다.
    pub fn new<P: AsRef<Path>>(root: P, exclude_patterns: &[String], target: BuildTarget) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        if !root.exists() {
            anyhow::bail!("Plugins directory not found: {:?}", root);
        }

        let filter = FileFilter::new(exclude_patterns)?;
        let resolver = ResourcePathResolver::new();
        let parser = ResourceParser::new(target);

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
            .map(|(key, paths)| self.parser.parse_resource(key, paths))
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
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

        // Command: md + yaml
        fs::write(cmd_dir.join("foo.md"), "# Foo Content")?;
        fs::write(cmd_dir.join("foo.yaml"), "key: val")?;
        // Exclude 대상
        fs::write(cmd_dir.join("test.tmp"), "temp")?;

        // Skill: SKILL.yaml + md
        fs::write(skill_dir.join("SKILL.yaml"), "desc: skill")?;
        fs::write(skill_dir.join("logic.md"), "prompt")?;

        let loader = ResourceLoader::new(&plugins_path, &["*.tmp".to_string()], BuildTarget::GeminiCli)?;
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
