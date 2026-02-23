mod config;
mod core;
mod emitter;
mod transformers;

use anyhow::Context;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "agb")]
#[command(about = "Agents Builder: Multi-agent workflow resource manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the agent resources based on agb.yaml
    Build {
        /// Optional path to the config file
        #[arg(short, long, default_value = "tests/fixtures/agb.yaml")]
        config: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { config } => {
            let config_path = std::path::Path::new(config);
            let root_dir = config_path.parent().unwrap_or(std::path::Path::new("."));

            println!("[1/5] Loading config: {}", config);
            let cfg = config::load_config(config)?;

            // 1. 모든 플러그인 파일 스캔
            println!("[2/5] Scanning and loading resources...");
            let plugins_dir = root_dir.join("plugins");
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
            println!(
                "[4/5] Transforming resources for target: {:?}...",
                cfg.target
            );
            let transformer = transformers::get_transformer(&cfg.target);
            let mut transformed_files = Vec::new();

            for res in registry.all_resources() {
                let transformed = transformer
                    .transform(res)
                    .with_context(|| format!("Failed to transform resource: {}", res.name()))?;
                transformed_files.push(transformed);
            }

            // AGENTS.md 처리 (Root System Prompt)
            let mut agents_md_path = root_dir.join("AGENTS.md");
            if !agents_md_path.exists() {
                agents_md_path = root_dir
                    .parent()
                    .unwrap_or(std::path::Path::new("."))
                    .join("AGENTS.md");
            }
            if !agents_md_path.exists() {
                agents_md_path = std::path::PathBuf::from("AGENTS.md");
            }

            if agents_md_path.exists() {
                println!("  - Found root system prompt: {}", agents_md_path.display());
                let content = std::fs::read_to_string(&agents_md_path)?;
                let transformed = transformer.transform_root_prompt(&content)?;
                transformed_files.push(transformed);
            }

            // 4. Emission
            println!("[5/5] Emitting files to {}...", root_dir.display());
            let emitter = emitter::Emitter::new(root_dir);
            emitter.clean()?;
            emitter.emit(&transformed_files)?;

            println!("\nBuild successful!");
            println!("  - Target: {:?}", cfg.target);
            println!("  - Resources: {} total", registry.len());
            println!("  - Files generated: {}", transformed_files.len());
        }
    }

    Ok(())
}
