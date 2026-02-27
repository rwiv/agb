use crate::core::Config;
use crate::loader;
use crate::loader::registry::Registry as LoaderRegistry;
use crate::transformer::Transformer;
use crate::transformer::TransformerFactory;
use log::info;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub struct AppContext {
    pub config: Config,
    pub registry: LoaderRegistry,
    pub transformer: Box<dyn Transformer>,
    pub source_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl AppContext {
    pub fn init(config_file: &str) -> anyhow::Result<Self> {
        let config_path = Path::new(config_file);
        if !config_path.exists() {
            anyhow::bail!("Config file not found: {}", config_file);
        }
        let output_dir = config_path.parent().unwrap_or(Path::new(".")).to_path_buf();

        info!("Loading config: {}", config_file);
        let cfg = crate::core::load_config(config_file)?;
        let source_dir = PathBuf::from(&cfg.source);

        if !source_dir.exists() {
            anyhow::bail!("Source directory does not exist: {}", source_dir.display());
        }

        // 설정으로부터 식별자 추출
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

        let exclude = cfg.exclude.as_ref().cloned().unwrap_or_default();

        // ResourceLoader를 통한 리소스 로드 및 필터링
        info!("Scanning and loading resources from {}...", source_dir.display());
        let loader = loader::ResourceLoader::new(&source_dir, &exclude, cfg.target)?;

        info!("Validating and registering resources...");
        let all_resources = loader.load()?;
        let mut registry = LoaderRegistry::new();

        for resource in all_resources {
            let identifier = format!("{}:{}", resource.plugin(), resource.name());
            if target_identifiers.contains(&identifier) {
                registry.register(resource)?;
            }
        }

        let transformer = TransformerFactory::create(&cfg.target);

        Ok(Self {
            config: cfg,
            registry,
            transformer,
            source_dir,
            output_dir,
        })
    }
}
