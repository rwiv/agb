use super::config;
use crate::core;
use crate::emitter;
use crate::transformers;
use anyhow::Context;
use std::path::{Path, PathBuf};

pub struct Builder {
    config_file: String,
    output_dir: PathBuf,
}

impl Builder {
    pub fn new(config_file: &str) -> Self {
        let config_path = Path::new(config_file);
        let output_dir = config_path.parent().unwrap_or(Path::new(".")).to_path_buf();

        Self {
            config_file: config_file.to_string(),
            output_dir,
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let config_path = Path::new(&self.config_file);
        if !config_path.exists() {
            anyhow::bail!("Config file not found: {}", self.config_file);
        }

        println!("[1/5] Loading config: {}", self.config_file);
        let cfg = config::load_config(&self.config_file)?;

        let source_dir = Path::new(&cfg.source);
        if !source_dir.exists() {
            anyhow::bail!("Source directory does not exist: {}", cfg.source);
        }

        // 1. 모든 플러그인 파일 스캔
        println!("[2/5] Scanning and loading resources from {}...", source_dir.display());
        let plugins_dir = source_dir.join("plugins");
        let exclude = cfg.exclude.unwrap_or_default();

        let files = core::loader::scan_plugins(&plugins_dir, &exclude)?;
        let all_resources = core::loader::load_resources(&plugins_dir, files)?;

        // 2. Registry 구축 및 agb.yaml에 명시된 리소스 필터링
        println!("[3/5] Validating and registering resources...");
        let mut registry = core::registry::Registry::new();

        let mut target_identifiers = std::collections::HashSet::new();
        if let Some(cmds) = &cfg.resources.commands {
            target_identifiers.extend(cmds);
        }
        if let Some(agents) = &cfg.resources.agents {
            target_identifiers.extend(agents);
        }
        if let Some(skills) = &cfg.resources.skills {
            target_identifiers.extend(skills);
        }

        for res in all_resources {
            let identifier = format!("{}:{}", res.plugin(), res.name());
            if target_identifiers.contains(&identifier) {
                registry.register(res)?;
            }
        }

        // 3. Transformation
        println!("[4/5] Transforming resources for target: {:?}...", cfg.target);
        let transformer = transformers::get_transformer(&cfg.target);
        let mut transformed_files = Vec::new();

        for res in registry.all_resources() {
            let transformed = transformer
                .transform(res)
                .with_context(|| format!("Failed to transform resource: {}", res.name()))?;
            transformed_files.push(transformed);
        }

        // AGENTS.md 처리 (Root System Prompt)
        let agents_md_path = source_dir.join("AGENTS.md");

        if agents_md_path.exists() {
            println!("  - Found root system prompt: {}", agents_md_path.display());
            let content = std::fs::read_to_string(&agents_md_path)?;
            let transformed = transformer.transform_root_prompt(&content)?;
            transformed_files.push(transformed);
        }

        // 4. Emission
        println!("[5/5] Emitting files to {}...", self.output_dir.display());
        let emitter = emitter::Emitter::new(&self.output_dir);
        emitter.clean()?;
        emitter.emit(&transformed_files)?;

        println!("Build successful!");
        println!("  - Target: {:?}", cfg.target);
        println!("  - Resources: {} total", registry.len());
        println!("  - Files generated: {}", transformed_files.len());

        Ok(())
    }
}
