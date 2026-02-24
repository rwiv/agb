use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// 리소스 식별을 위한 키
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ResourceKey {
    pub plugin: String,
    pub r_type: String,
    pub name: String,
}

/// 리소스를 구성하는 파일 경로 그룹
#[derive(Debug, Default, Clone)]
pub struct ResourcePaths {
    pub md: Option<PathBuf>,
    pub metadata: Option<PathBuf>,
}

/// 파일 경로를 분석하여 리소스별로 그룹화하는 객체입니다.
pub struct ResourcePathResolver;

impl ResourcePathResolver {
    pub fn new() -> Self {
        Self
    }

    /// 파일 목록을 받아 리소스 키와 경로 그룹으로 변환합니다.
    pub fn resolve(&self, root: &Path, files: Vec<PathBuf>) -> Result<HashMap<ResourceKey, ResourcePaths>> {
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

            let plugin = components[0].clone();
            let r_type = components[1].clone();

            if r_type == "skills" {
                self.resolve_skill(&mut groups, plugin, r_type, &components, path)?;
            } else if r_type == "commands" || r_type == "agents" {
                self.resolve_command_agent(&mut groups, plugin, r_type, &components, path)?;
            }
        }

        Ok(groups)
    }

    fn resolve_skill(
        &self,
        groups: &mut HashMap<ResourceKey, ResourcePaths>,
        plugin: String,
        r_type: String,
        components: &[String],
        path: PathBuf,
    ) -> Result<()> {
        // Skill 특수 처리: [plugin]/skills/[skill_name]/...
        if components.len() >= 4 {
            let skill_name = components[2].clone();
            let file_name = components[3].clone();

            let entry = groups.entry(key).or_default();

            let is_metadata = (file_name == format!("{}.json", skill_name))
                || (file_name == format!("{}.yaml", skill_name))
                || (file_name == format!("{}.yml", skill_name));

            if is_metadata {
                if entry.metadata.is_some() {
                    anyhow::bail!(
                        "Conflict: Multiple metadata formats found for skill '{}' in plugin '{}'",
                        skill_name,
                        plugin
                    );
                }
                entry.metadata = Some(path);
            } else if file_name.ends_with(".md") {
                // 메인 마크다운 파일 (관례상 SKILL.md 또는 README.md 권장)
                entry.md = Some(path);
            }
        }
        Ok(())
    }

    fn resolve_command_agent(
        &self,
        groups: &mut HashMap<ResourceKey, ResourcePaths>,
        plugin: String,
        r_type: String,
        _components: &[String],
        path: PathBuf,
    ) -> Result<()> {
        // Command/Agent 처리: [plugin]/[type]/[name].{md,json,yaml,yml}
        let file_stem = path.file_stem().unwrap().to_string_lossy().into_owned();
        let extension = path.extension().unwrap_or_default().to_string_lossy().into_owned();

        let entry = groups.entry(key).or_default();

        if extension == "md" {
            entry.md = Some(path);
        } else if extension == "json" || extension == "yaml" || extension == "yml" {
            if entry.metadata.is_some() {
                anyhow::bail!(
                    "Conflict: Multiple metadata formats found for resource '{}' in plugin '{}'",
                    file_stem,
                    plugin
                );
            }
            entry.metadata = Some(path);
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
            PathBuf::from("/root/p1/commands/foo.json"),
            PathBuf::from("/root/p2/skills/task/task.yaml"),
            PathBuf::from("/root/p2/skills/task/logic.md"),
            PathBuf::from("/root/p1/agents/bot.md"),
        ];

        let resolver = ResourcePathResolver::new();
        let groups = resolver.resolve(root, files)?;

        // p1:commands:foo -> (Some(foo.md), Some(foo.json))
        let foo_key = ResourceKey {
            plugin: "p1".to_string(),
            r_type: "commands".to_string(),
            name: "foo".to_string(),
        };
        let paths = groups.get(&foo_key).unwrap();
        assert!(paths.md.as_ref().unwrap().ends_with("foo.md"));
        assert!(paths.metadata.as_ref().unwrap().ends_with("foo.json"));

        // p2:skills:task -> (Some(logic.md), Some(task.yaml))
        let task_key = ResourceKey {
            plugin: "p2".to_string(),
            r_type: "skills".to_string(),
            name: "task".to_string(),
        };
        let paths = groups.get(&task_key).unwrap();
        assert!(paths.md.as_ref().unwrap().ends_with("logic.md"));
        assert!(paths.metadata.as_ref().unwrap().ends_with("task.yaml"));

        // p1:agents:bot -> (Some(bot.md), None)
        let bot_key = ResourceKey {
            plugin: "p1".to_string(),
            r_type: "agents".to_string(),
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
            PathBuf::from("/root/p1/commands/foo.json"),
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
