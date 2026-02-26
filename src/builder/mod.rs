pub mod emitter;

use crate::loader::ResourceLoader;
use crate::transformer;
use anyhow::Context;
use std::path::Path;

use self::emitter::Emitter;
use crate::core::{PLUGINS_DIR_NAME, TransformedResource, Config};

#[derive(Default)]
pub struct BuildExecutor;

impl BuildExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, cfg: &Config, source_dir: &Path, output_dir: &Path) -> anyhow::Result<()> {
        if !source_dir.exists() {
            anyhow::bail!("Source directory does not exist: {}", source_dir.display());
        }

        // 1. 모든 플러그인 파일 스캔 및 로드
        println!("Scanning and loading resources from {}...", source_dir.display());
        let plugins_dir = source_dir.join(PLUGINS_DIR_NAME);
        let exclude = cfg.exclude.as_ref().cloned().unwrap_or_default();

        let loader = ResourceLoader::new(&plugins_dir, &exclude, cfg.target)?;

        // 2. agb.yaml에 명시된 리소스 필터링 및 Registry 구축
        println!("Validating and registering resources...");
        let mut target_identifiers = std::collections::HashSet::new();
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

        // 3. Transformation
        println!("Transforming resources for target: {:?}...", cfg.target);
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
        println!("Emitting files to {}...", output_dir.display());
        let emitter = Emitter::new(output_dir);
        emitter.clean()?;
        emitter.emit(&transformed_resources)?;

        println!("Build successful!");
        println!("  - Target: {:?}", cfg.target);
        println!("  - Resources: {} total", registry.len());

        Ok(())
    }
}
