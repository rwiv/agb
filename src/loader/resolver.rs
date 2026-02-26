use crate::core::{DIR_AGENTS, DIR_COMMANDS, DIR_SKILLS, EXT_MD, EXT_YAML, EXT_YML, SKILL_MD};
use crate::loader::{ScannedPaths, ScannedResource};
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// 경로 분석에 필요한 정보를 담는 컨텍스트 구조체입니다.
struct ResolveContext {
    plugin: String,
    r_type: String,
    components: Vec<String>,
    path: PathBuf,
}

/// 파일 경로를 분석하여 리소스별로 그룹화하는 객체입니다.
pub struct ResourcePathResolver;

impl ResourcePathResolver {
    pub fn new() -> Self {
        Self
    }

    /// 파일 목록을 받아 스캔된 리소스 목록으로 변환합니다.
    pub fn resolve(&self, root: &Path, files: Vec<PathBuf>) -> Result<Vec<ScannedResource>> {
        let mut command_groups: HashMap<(String, String), (Option<PathBuf>, Option<PathBuf>)> = HashMap::new();
        let mut agent_groups: HashMap<(String, String), (Option<PathBuf>, Option<PathBuf>)> = HashMap::new();
        let mut skill_groups: HashMap<(String, String), (Option<PathBuf>, Option<PathBuf>, Vec<PathBuf>)> =
            HashMap::new();

        for path in files {
            let relative = path.strip_prefix(root).unwrap_or(&path);
            let components: Vec<_> = relative
                .components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect();

            if components.len() < 3 {
                continue; // [plugin]/[type]/[name] 구조가 아니면 무시
            }

            let ctx = ResolveContext {
                plugin: components[0].clone(),
                r_type: components[1].clone(),
                components,
                path,
            };

            if ctx.r_type == DIR_COMMANDS {
                self.resolve_default(&mut command_groups, ctx)?;
            } else if ctx.r_type == DIR_AGENTS {
                self.resolve_default(&mut agent_groups, ctx)?;
            } else if ctx.r_type == DIR_SKILLS {
                self.resolve_skill(&mut skill_groups, ctx)?;
            }
        }

        let mut results = Vec::new();

        for ((plugin, name), (md, metadata)) in command_groups {
            results.push(ScannedResource {
                plugin,
                name,
                paths: ScannedPaths::Command { md, metadata },
            });
        }

        for ((plugin, name), (md, metadata)) in agent_groups {
            results.push(ScannedResource {
                plugin,
                name,
                paths: ScannedPaths::Agent { md, metadata },
            });
        }

        for ((plugin, name), (md, metadata, extras)) in skill_groups {
            results.push(ScannedResource {
                plugin,
                name,
                paths: ScannedPaths::Skill { md, metadata, extras },
            });
        }

        Ok(results)
    }

    fn resolve_default(
        &self,
        groups: &mut HashMap<(String, String), (Option<PathBuf>, Option<PathBuf>)>,
        ctx: ResolveContext,
    ) -> Result<()> {
        let file_stem = ctx.path.file_stem().unwrap().to_string_lossy().into_owned();
        let extension = ctx.path.extension().unwrap_or_default().to_string_lossy().into_owned();

        let key = (ctx.plugin.clone(), file_stem.clone());
        let entry = groups.entry(key).or_insert((None, None));

        if extension == EXT_MD[1..] {
            entry.0 = Some(ctx.path);
        } else if self.is_metadata_extension(&extension) {
            self.validate_metadata_uniqueness(&entry.1, &file_stem, &ctx.plugin)?;
            entry.1 = Some(ctx.path);
        }
        Ok(())
    }

    fn resolve_skill(
        &self,
        groups: &mut HashMap<(String, String), (Option<PathBuf>, Option<PathBuf>, Vec<PathBuf>)>,
        ctx: ResolveContext,
    ) -> Result<()> {
        if ctx.components.len() < 4 {
            return Ok(());
        }

        let skill_name = ctx.components[2].clone();
        let file_name = ctx.components[3].clone();

        let key = (ctx.plugin.clone(), skill_name.clone());
        let entry = groups.entry(key).or_insert((None, None, Vec::new()));

        let path_for_ext = Path::new(&file_name);
        let stem = path_for_ext.file_stem().and_then(|s| s.to_str());
        let ext = path_for_ext.extension().and_then(|s| s.to_str()).unwrap_or_default();

        let skill_md_stem = Path::new(SKILL_MD).file_stem().and_then(|s| s.to_str());

        if stem == skill_md_stem && self.is_metadata_extension(ext) {
            self.validate_metadata_uniqueness(&entry.1, &skill_name, &ctx.plugin)?;
            entry.1 = Some(ctx.path);
        } else if file_name == SKILL_MD {
            entry.0 = Some(ctx.path);
        } else {
            // SKILL.md나 SKILL.yaml이 아닌 모든 파일은 extras로 분류
            entry.2.push(ctx.path);
        }

        Ok(())
    }

    fn is_metadata_extension(&self, ext: &str) -> bool {
        ext == &EXT_YAML[1..] || ext == &EXT_YML[1..]
    }

    fn validate_metadata_uniqueness(&self, existing: &Option<PathBuf>, name: &str, plugin: &str) -> Result<()> {
        if existing.is_some() {
            anyhow::bail!(
                "Conflict: Multiple metadata formats found for resource '{}' in plugin '{}'",
                name,
                plugin
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_path_resolver_resolve() -> Result<()> {
        let root = Path::new("/root");
        let files = vec![
            PathBuf::from("/root/p1/commands/foo.md"),
            PathBuf::from("/root/p1/commands/foo.yaml"),
            PathBuf::from("/root/p2/skills/task/SKILL.yaml"),
            PathBuf::from("/root/p2/skills/task/SKILL.md"),
            PathBuf::from("/root/p2/skills/task/extra.txt"),
            PathBuf::from("/root/p1/agents/bot.md"),
        ];

        let resolver = ResourcePathResolver::new();
        let results = resolver.resolve(root, files)?;

        assert_eq!(results.len(), 3);

        for res in results {
            match res.paths {
                ScannedPaths::Command { md, metadata } if res.name == "foo" => {
                    assert_eq!(res.plugin, "p1");
                    assert!(md.unwrap().ends_with("foo.md"));
                    assert!(metadata.unwrap().ends_with("foo.yaml"));
                }
                ScannedPaths::Skill { md, metadata, extras } if res.name == "task" => {
                    assert_eq!(res.plugin, "p2");
                    assert!(md.unwrap().ends_with("SKILL.md"));
                    assert!(metadata.unwrap().ends_with("SKILL.yaml"));
                    assert_eq!(extras.len(), 1);
                    assert!(extras[0].ends_with("extra.txt"));
                }
                ScannedPaths::Agent { md, metadata } if res.name == "bot" => {
                    assert_eq!(res.plugin, "p1");
                    assert!(md.unwrap().ends_with("bot.md"));
                    assert!(metadata.is_none());
                }
                _ => panic!("Unexpected resource: {:?}", res),
            }
        }

        Ok(())
    }

    #[test]
    fn test_resource_path_resolver_conflict_error() {
        let root = Path::new("/root");
        let files = vec![
            PathBuf::from("/root/p1/commands/foo.yml"),
            PathBuf::from("/root/p1/commands/foo.yaml"),
        ];

        let resolver = ResourcePathResolver::new();
        let result = resolver.resolve(root, files);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Multiple metadata formats found")
        );
    }
}
