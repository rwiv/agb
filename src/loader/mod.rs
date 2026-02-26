pub mod filter;
pub mod parser;
pub mod registry;
pub mod resolver;

use crate::core::{BuildTarget, Config, PLUGINS_DIR_NAME, Resource};
use anyhow::Result;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use self::filter::FileFilter;
use self::parser::ResourceParser;
use self::registry::Registry;
use self::resolver::ResourcePathResolver;

/// Config를 기반으로 Registry를 구축합니다.
pub fn load_registry_from_config(cfg: &Config, source_dir: &Path) -> Result<Registry> {
    if !source_dir.exists() {
        anyhow::bail!("Source directory does not exist: {}", source_dir.display());
    }

    println!("Scanning and loading resources from {}...", source_dir.display());
    let plugins_dir = source_dir.join(PLUGINS_DIR_NAME);
    let exclude = cfg.exclude.as_ref().cloned().unwrap_or_default();

    let loader = ResourceLoader::new(&plugins_dir, &exclude, cfg.target)?;

    println!("Validating and registering resources...");
    let mut target_identifiers = HashSet::new();
    if let Some(cmds) = &cfg.resources.commands {
        target_identifiers.extend(cmds.clone());
    }
    if let Some(agents) = &cfg.resources.agents {
        target_identifiers.extend(agents.clone());
    }
    if let Some(skills) = &cfg.resources.skills {
        target_identifiers.extend(skills.clone());
    }

    loader.load_registry(&target_identifiers)
}

/// 스캔된 리소스 정보를 담는 내부 구조체
#[derive(Debug, Clone)]
pub(crate) struct ScannedResource {
    pub(crate) plugin: String,
    pub(crate) name: String,
    pub(crate) paths: ScannedPaths,
}

/// 리소스 타입별 파일 경로 구성
#[derive(Debug, Clone)]
pub(crate) enum ScannedPaths {
    Command {
        md: Option<PathBuf>,
        metadata: Option<PathBuf>,
    },
    Agent {
        md: Option<PathBuf>,
        metadata: Option<PathBuf>,
    },
    Skill {
        md: Option<PathBuf>,
        metadata: Option<PathBuf>,
        extras: Vec<PathBuf>,
    },
}

/// 플러그인 디렉터리를 탐색하고 리소스를 로드하는 객체입니다.
struct ResourceLoader {
    root: PathBuf,
    filter: FileFilter,
    resolver: ResourcePathResolver,
    parser: ResourceParser,
}

impl ResourceLoader {
    /// 새로운 ResourceLoader 인스턴스를 생성합니다.
    fn new<P: AsRef<Path>>(root: P, exclude_patterns: &[String], target: BuildTarget) -> Result<Self> {
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
    fn load(&self) -> Result<Vec<Resource>> {
        let files = self.scan()?;
        let scanned_resources = self.resolver.resolve(&self.root, files)?;

        scanned_resources
            .into_iter()
            .map(|scanned| self.parser.parse_resource(scanned))
            .collect()
    }

    /// 타겟 식별자 목록에 해당하는 리소스만 로드하여 Registry를 구축합니다.
    fn load_registry(&self, target_identifiers: &HashSet<String>) -> Result<Registry> {
        let all_resources = self.load()?;
        let mut registry = Registry::new();

        for resource in all_resources {
            let identifier = format!("{}:{}", resource.plugin(), resource.name());
            if target_identifiers.contains(&identifier) {
                registry.register(resource)?;
            }
        }

        Ok(registry)
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
        fs::write(cmd_dir.join("foo.yaml"), "gemini-cli:\n  key: val")?;
        // Exclude 대상
        fs::write(cmd_dir.join("test.tmp"), "temp")?;

        // Skill: SKILL.yaml + md
        fs::write(skill_dir.join("SKILL.yaml"), "gemini-cli:\n  desc: skill")?;
        fs::write(skill_dir.join("SKILL.md"), "prompt")?;

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
                Resource::Skill(s) if s.base.name == "my_skill" => {
                    assert_eq!(s.base.plugin, "plugin_b");
                    assert_eq!(s.base.metadata["desc"], "skill");
                    assert!(s.base.content.contains("prompt"));
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
