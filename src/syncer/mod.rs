pub mod diff;
pub mod syncer;

use crate::builder::config;
use crate::core::{Registry, PLUGINS_DIR_NAME};
use crate::loader::ResourceLoader;
use crate::syncer::syncer::Syncer;
use crate::transformer::TransformerFactory;
use anyhow::Result;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub struct SyncExecutor {
    config_file: String,
    output_dir: PathBuf,
}

impl SyncExecutor {
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
        let syncer = Syncer::new(self.output_dir.clone(), exclude);

        for res in registry.all_resources() {
            syncer.sync(res, transformer.as_ref())?;
        }

        println!("Sync successful!");
        Ok(())
    }
}
