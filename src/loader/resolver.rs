use crate::core::{
    ResourceKey, ResourcePaths, DIR_AGENTS, DIR_COMMANDS, DIR_SKILLS, EXT_MD, EXT_YAML, EXT_YML,
    SKILL_MD,
};
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

    /// 파일 목록을 받아 리소스 키와 경로 그룹으로 변환합니다.
    pub fn resolve(
        &self,
        root: &Path,
        files: Vec<PathBuf>,
    ) -> Result<HashMap<ResourceKey, ResourcePaths>> {
        let mut groups: HashMap<ResourceKey, ResourcePaths> = HashMap::new();

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

            if ctx.r_type == DIR_COMMANDS || ctx.r_type == DIR_AGENTS {
                self.resolve_default(&mut groups, ctx)?;
            } else if ctx.r_type == DIR_SKILLS {
                self.resolve_skill(&mut groups, ctx)?;
            }
        }

        Ok(groups)
    }

    fn resolve_default(
        &self,
        groups: &mut HashMap<ResourceKey, ResourcePaths>,
        ctx: ResolveContext,
    ) -> Result<()> {
        // Command/Agent 처리: [plugin]/[type]/[name].{md,yaml,yml}
        let file_stem = ctx.path.file_stem().unwrap().to_string_lossy().into_owned();
        let extension = ctx
            .path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();

        let key = ResourceKey {
            plugin: ctx.plugin.clone(),
            r_type: ctx.r_type,
            name: file_stem.clone(),
        };
        let entry = groups.entry(key).or_default();

        if extension == &EXT_MD[1..] {
            entry.md = Some(ctx.path);
        } else if self.is_metadata_extension(&extension) {
            self.validate_metadata_uniqueness(&entry.metadata, &file_stem, &ctx.plugin)?;
            entry.metadata = Some(ctx.path);
        }
        Ok(())
    }

    fn resolve_skill(
        &self,
        groups: &mut HashMap<ResourceKey, ResourcePaths>,
        ctx: ResolveContext,
    ) -> Result<()> {
        // Skill 특수 처리: [plugin]/skills/[skill_name]/...
        // 4개 미만이면 유효한 스킬 파일 구조가 아니므로 즉시 종료
        if ctx.components.len() < 4 {
            return Ok(());
        }

        let skill_name = ctx.components[2].clone();
        let file_name = ctx.components[3].clone();

        let key = ResourceKey {
            plugin: ctx.plugin.clone(),
            r_type: ctx.r_type.clone(),
            name: skill_name.clone(),
        };
        let entry = groups.entry(key).or_default();

        let path_for_ext = Path::new(&file_name);
        let stem = path_for_ext.file_stem().and_then(|s| s.to_str());
        let ext = path_for_ext
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        let skill_md_stem = Path::new(SKILL_MD).file_stem().and_then(|s| s.to_str());

        if stem == skill_md_stem && self.is_metadata_extension(ext) {
            self.validate_metadata_uniqueness(&entry.metadata, &skill_name, &ctx.plugin)?;
            entry.metadata = Some(ctx.path);
        } else if file_name.ends_with(EXT_MD) {
            // 메인 마크다운 파일 (관례상 SKILL.md 또는 README.md 권장)
            entry.md = Some(ctx.path);
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
            PathBuf::from("/root/p2/skills/task/logic.md"),
            PathBuf::from("/root/p1/agents/bot.md"),
        ];

        let resolver = ResourcePathResolver::new();
        let groups = resolver.resolve(root, files)?;

        // p1:commands:foo -> (Some(foo.md), Some(foo.yaml))
        let foo_key = ResourceKey {
            plugin: "p1".to_string(),
            r_type: DIR_COMMANDS.to_string(),
            name: "foo".to_string(),
        };
        let paths = groups.get(&foo_key).unwrap();
        assert!(paths.md.as_ref().unwrap().ends_with("foo.md"));
        assert!(paths.metadata.as_ref().unwrap().ends_with("foo.yaml"));

        // p2:skills:task -> (Some(logic.md), Some(SKILL.yaml))
        let task_key = ResourceKey {
            plugin: "p2".to_string(),
            r_type: DIR_SKILLS.to_string(),
            name: "task".to_string(),
        };
        let paths = groups.get(&task_key).unwrap();
        assert!(paths.md.as_ref().unwrap().ends_with("logic.md"));
        assert!(paths.metadata.as_ref().unwrap().ends_with("SKILL.yaml"));

        // p1:agents:bot -> (Some(bot.md), None)
        let bot_key = ResourceKey {
            plugin: "p1".to_string(),
            r_type: DIR_AGENTS.to_string(),
            name: "bot".to_string(),
        };
        let paths = groups.get(&bot_key).unwrap();
        assert!(paths.md.as_ref().unwrap().ends_with("bot.md"));
        assert!(paths.metadata.is_none());

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
