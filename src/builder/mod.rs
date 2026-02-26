pub mod config;
pub mod emitter;

use crate::loader::ResourceLoader;
use crate::transformer;
use anyhow::Context;
use std::path::{Path, PathBuf};

use self::emitter::Emitter;
use crate::core::{Registry, TransformedResource, PLUGINS_DIR_NAME};

pub struct BuildExecutor {
    config_file: String,
    output_dir: PathBuf,
}

impl BuildExecutor {
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

        // 1. 모든 플러그인 파일 스캔 및 로드
        println!("[2/5] Scanning and loading resources from {}...", source_dir.display());
        let plugins_dir = source_dir.join(PLUGINS_DIR_NAME);
        let exclude = cfg.exclude.unwrap_or_default();

        let loader = ResourceLoader::new(&plugins_dir, &exclude, cfg.target)?;
        let all_resources = loader.load()?;

        // 2. agb.yaml에 명시된 리소스 필터링 및 Registry 구축
        println!("[3/5] Validating and registering resources...");

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

        let mut registry = Registry::new();
        for resource in all_resources {
            let identifier = format!("{}:{}", resource.plugin(), resource.name());
            if target_identifiers.contains(&identifier) {
                registry.register(resource)?;
            }
        }

        // 3. Transformation
        println!("[4/5] Transforming resources for target: {:?}...", cfg.target);
        let transformer = transformer::TransformerFactory::create(&cfg.target);

        let mut transformed_resources = Vec::new();
        for res in registry.all_resources() {
            let transformed_file = transformer
                .transform(res)
                .with_context(|| format!("Failed to transform resource: {}", res.name()))?;

            transformed_resources.push(TransformedResource {
                files: vec![transformed_file],
                extras: res.extras(),
            });
        }

        // AGENTS.md 처리 (Root System Prompt)
        let agents_md_path = source_dir.join(crate::core::AGENTS_MD);
        if agents_md_path.exists() {
            println!("  - Found root system prompt: {}", agents_md_path.display());
            let raw_content = std::fs::read_to_string(&agents_md_path)?;
            let (_fm, pure_content) = crate::utils::yaml::extract_frontmatter(&raw_content);
            let transformed_file = transformer.transform_root_prompt(&pure_content)?;

            transformed_resources.push(TransformedResource {
                files: vec![transformed_file],
                extras: Vec::new(),
            });
        }

        // 4. Emission
        println!("[5/5] Emitting files to {}...", self.output_dir.display());
        let emitter = Emitter::new(&self.output_dir);
        emitter.clean()?;
        emitter.emit(&transformed_resources)?;

        println!("Build successful!");
        println!("  - Target: {:?}", cfg.target);
        println!("  - Resources: {} total", registry.len());

        Ok(())
    }
}
