pub mod diff;

use crate::builder::config;
use crate::core::{Registry, Resource, SKILL_MD};
use crate::loader::ResourceLoader;
use crate::transformer::{Transformer, TransformerFactory};
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

const PLUGINS_DIR_NAME: &str = "plugins";

pub struct Syncer {
    config_file: String,
    output_dir: PathBuf,
}

impl Syncer {
    pub fn new(config_file: &str) -> Self {
        let config_path = Path::new(config_file);
        let output_dir = config_path.parent().unwrap_or(Path::new(".")).to_path_buf();

        Self {
            config_file: config_file.to_string(),
            output_dir,
        }
    }

    pub fn run(&self) -> Result<()> {
        let config_path = Path::new(&self.config_file);
        if !config_path.exists() {
            anyhow::bail!("Config file not found: {}", self.config_file);
        }

        println!("[1/4] Loading config for sync: {}", self.config_file);
        let cfg = config::load_config(&self.config_file)?;

        let source_dir = Path::new(&cfg.source);
        if !source_dir.exists() {
            anyhow::bail!("Source directory does not exist: {}", cfg.source);
        }

        // 1. 소스 레지스트리 구축 (ResourceLoader 활용)
        println!("[2/4] Loading source registry from {}...", source_dir.display());
        let plugins_dir = source_dir.join(PLUGINS_DIR_NAME);
        let exclude = cfg.exclude.clone().unwrap_or_default();

        let loader = ResourceLoader::new(&plugins_dir, &exclude, cfg.target)?;
        let all_resources = loader.load()?;

        let mut target_identifiers = HashSet::new();
        if let Some(cmds) = &cfg.resources.commands {
            target_identifiers.extend(cmds);
        }
        if let Some(agents) = &cfg.resources.agents {
            target_identifiers.extend(agents);
        }
        if let Some(skills) = &cfg.resources.skills {
            target_identifiers.extend(skills);
        }

        let mut registry = Registry::new();
        for resource in all_resources {
            let identifier = format!("{}:{}", resource.plugin(), resource.name());
            if target_identifiers.contains(&identifier) {
                registry.register(resource)?;
            }
        }

        // 2. Transformer 및 Registry 순회 동기화
        println!("[3/4] Syncing target changes to source for target: {:?}...", cfg.target);
        let transformer = TransformerFactory::create(&cfg.target);

        for res in registry.all_resources() {
            self.sync_resource(res, transformer.as_ref(), &exclude)?;
        }

        println!("Sync successful!");
        Ok(())
    }

    fn sync_resource(&self, resource: &Resource, transformer: &dyn Transformer, exclude: &[String]) -> Result<()> {
        // 타겟 파일 경로 결정 (Transformation을 통해 생성된 경로와 일치해야 함)
        let transformed = transformer
            .transform(resource)
            .with_context(|| format!("Failed to determine target path for {}", resource.name()))?;

        let target_path = self.output_dir.join(&transformed.path);
        if !target_path.exists() {
            return Ok(()); // 타겟 파일이 없으면 변경사항도 없는 것으로 간주
        }

        println!("  Checking resource: {}/{}", resource.r_type(), resource.name());

        // 타겟 파일 내용 읽기
        let target_content = fs::read_to_string(&target_path)
            .with_context(|| format!("Failed to read target file: {:?}", target_path))?;

        // 역변환 (Detransform)
        let detransformed = transformer
            .detransform(resource.r_type(), &target_content)
            .with_context(|| format!("Failed to detransform target file: {:?}", target_path))?;

        // 소스 정보 가져오기
        let (source_path, current_content, current_metadata) = match resource {
            Resource::Command(d) | Resource::Agent(d) => (&d.source_path, &d.content, &d.metadata),
            Resource::Skill(s) => (&s.base.source_path, &s.base.content, &s.base.metadata),
        };

        let mut source_file_content = match resource {
            Resource::Command(_) | Resource::Agent(_) => fs::read_to_string(source_path)?,
            Resource::Skill(_) => fs::read_to_string(source_path.join(SKILL_MD))?,
        };

        let mut changed = false;

        // 1. Description 동기화
        let old_desc = current_metadata["description"].as_str().unwrap_or_default();
        let new_desc = detransformed.metadata["description"].as_str().unwrap_or_default();

        if old_desc != new_desc {
            source_file_content = diff::update_description(&source_file_content, new_desc);
            changed = true;
            println!("    - Updated description in source");
        }

        // 2. Content 동기화
        if diff::diff_content(current_content, &detransformed.content) {
            source_file_content = diff::replace_content(&source_file_content, &detransformed.content);
            changed = true;
            println!("    - Updated content in source");
        }

        // 소스 파일 쓰기
        if changed {
            let write_path = match resource {
                Resource::Command(_) | Resource::Agent(_) => source_path.clone(),
                Resource::Skill(_) => source_path.join(SKILL_MD),
            };
            fs::write(&write_path, source_file_content)?;
        }

        // 3. Skill ExtraFiles 동기화
        if let Resource::Skill(_) = resource {
            let target_skill_dir = target_path.parent().unwrap();
            diff::sync_skill_dir(source_path, target_skill_dir, exclude)?;
        }

        Ok(())
    }
}
