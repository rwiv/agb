use crate::core::resource::{Resource, ResourceData};
use anyhow::{Context, Result};
use glob::Pattern;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// plugins 디렉터리를 탐색하여 유효한 파일 경로 리스트를 반환합니다.
pub fn scan_plugins<P: AsRef<Path>>(
    root: P,
    exclude_patterns: &[String],
) -> Result<Vec<PathBuf>> {
    let root = root.as_ref();
    if !root.exists() {
        anyhow::bail!("Plugins directory not found: {:?}", root);
    }

    let mut patterns = Vec::new();
    for p in exclude_patterns {
        patterns.push(Pattern::new(p).with_context(|| format!("Invalid glob pattern: {}", p))?);
    }

    let mut files = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        if path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
        {
            continue;
        }

        let relative_path = path.strip_prefix(root).unwrap_or(path);
        let mut is_excluded = false;
        for pattern in &patterns {
            if pattern.matches_path(relative_path) {
                is_excluded = true;
                break;
            }
        }

        if !is_excluded {
            files.push(path.to_path_buf());
        }
    }

    Ok(files)
}

/// 스캔된 파일들을 Resource 객체들로 로드합니다.
pub fn load_resources<P: AsRef<Path>>(
    root: P,
    files: Vec<PathBuf>,
) -> Result<Vec<Resource>> {
    let root = root.as_ref();
    let mut resources = Vec::new();
    
    // 파일들을 타입별/플러그인별/이름별로 그룹화하기 위한 맵
    // Key: (plugin_name, resource_type, resource_name)
    // Value: (md_path, json_path)
    let mut groups: HashMap<(String, String, String), (Option<PathBuf>, Option<PathBuf>)> = HashMap::new();

    for path in files {
        let relative = path.strip_prefix(root).unwrap_or(&path);
        let components: Vec<_> = relative.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect();
        
        if components.len() < 3 {
            continue; // [plugin]/[type]/[name] 구조가 아니면 무시
        }

        let plugin = components[0].clone();
        let r_type = components[1].clone();
        
        if r_type == "skills" {
            // Skill 특수 처리: plugins/[plugin]/skills/[skill_name]/...
            if components.len() >= 4 {
                let skill_name = components[2].clone();
                let file_name = components[3].clone();
                let entry = groups.entry((plugin, r_type, skill_name)).or_insert((None, None));
                if file_name == "METADATA.json" {
                    entry.1 = Some(path);
                } else if file_name.ends_with(".md") {
                    // 메인 마크다운 파일 (보통 skill_name.md 또는 README.md)
                    entry.0 = Some(path);
                }
            }
        } else if r_type == "commands" || r_type == "agents" {
            // Command/Agent 처리: plugins/[plugin]/[type]/[name].{md,json}
            let file_stem = path.file_stem().unwrap().to_string_lossy().into_owned();
            let extension = path.extension().unwrap_or_default().to_string_lossy().into_owned();
            
            let entry = groups.entry((plugin, r_type, file_stem)).or_insert((None, None));
            if extension == "md" {
                entry.0 = Some(path);
            } else if extension == "json" {
                entry.1 = Some(path);
            }
        }
    }

    for ((plugin, r_type, name), (md_path, json_path)) in groups {
        let content = if let Some(p) = md_path {
            fs::read_to_string(p)?
        } else {
            String::new()
        };

        let metadata = if let Some(p) = json_path {
            let json_str = fs::read_to_string(p)?;
            serde_json::from_str(&json_str).with_context(|| format!("Failed to parse JSON for resource: {}/{}", r_type, name))?
        } else {
            Value::Null
        };

        let data = ResourceData {
            name: name.clone(),
            plugin: plugin.clone(),
            content,
            metadata,
        };

        match r_type.as_str() {
            "commands" => resources.push(Resource::Command(data)),
            "agents" => resources.push(Resource::Agent(data)),
            "skills" => resources.push(Resource::Skill(data)),
            _ => {}
        }
    }

    Ok(resources)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::resource::Resource;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_scan_and_load_resources() -> Result<()> {
        let dir = tempdir()?;
        let plugins_path = dir.path().join("plugins");
        
        // 1. 샘플 구조 생성
        let cmd_dir = plugins_path.join("plugin_a/commands");
        let skill_dir = plugins_path.join("plugin_b/skills/my_skill");
        fs::create_dir_all(&cmd_dir)?;
        fs::create_dir_all(&skill_dir)?;

        // Command: md + json
        fs::write(cmd_dir.join("foo.md"), "# Foo Content")?;
        fs::write(cmd_dir.join("foo.json"), "{\"key\": \"val\"}")?;
        // Exclude 대상
        fs::write(cmd_dir.join("test.tmp"), "temp")?;
        
        // Skill: METADATA.json + md
        fs::write(skill_dir.join("METADATA.json"), "{\"desc\": \"skill\"}")?;
        fs::write(skill_dir.join("logic.md"), "prompt")?;

        // 2. 스캔 테스트
        let exclude = vec!["*.tmp".to_string()];
        let files = scan_plugins(&plugins_path, &exclude)?;
        // foo.md, foo.json, METADATA.json, logic.md 총 4개 (test.tmp 제외)
        assert_eq!(files.len(), 4);

        // 3. 로드 테스트
        let resources = load_resources(&plugins_path, files)?;
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
