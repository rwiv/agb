pub mod patcher;
pub mod planner;
pub mod skill;
pub mod sync;

use crate::core::{PLUGINS_DIR_NAME, Config};
use crate::loader::ResourceLoader;
use crate::syncer::sync::Syncer;
use crate::transformer::TransformerFactory;
use anyhow::Result;
use std::collections::HashSet;
use std::path::Path;

#[derive(Default)]
pub struct SyncExecutor;

impl SyncExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, cfg: &Config, source_dir: &Path, output_dir: &Path) -> Result<()> {
        if !source_dir.exists() {
            anyhow::bail!("Source directory does not exist: {}", source_dir.display());
        }

        // 1. 소스 레지스트리 구축 (ResourceLoader 활용)
        println!("Loading source registry from {}...", source_dir.display());
        let plugins_dir = source_dir.join(PLUGINS_DIR_NAME);
        let exclude = cfg.exclude.clone().unwrap_or_default();

        let loader = ResourceLoader::new(&plugins_dir, &exclude, cfg.target)?;

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

        let registry = loader.load_registry(&target_identifiers)?;

        // 2. Transformer 및 Registry 순회 동기화
        println!("Syncing target changes to source for target: {:?}...", cfg.target);
        let transformer = TransformerFactory::create(&cfg.target);
        let syncer = Syncer::new(output_dir.to_path_buf(), exclude);

        for res in registry.all_resources() {
            syncer.sync(res, transformer.as_ref())?;
        }

        println!("Sync successful!");
        Ok(())
    }
}
