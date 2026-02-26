mod builder;
mod core;
mod loader;
mod syncer;
mod transformer;
mod utils;

use builder::Builder;
use clap::{Parser, Subcommand};
use std::path::Path;
use syncer::Syncer;

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
        #[arg(short, long)]
        config: Option<String>,
    },
    /// Sync changes from target back to source
    Sync {
        /// Optional path to the config file
        #[arg(short, long)]
        config: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config_file = match &cli.command {
        Commands::Build { config } => config.as_deref().unwrap_or("agb.yaml"),
        Commands::Sync { config } => config.as_deref().unwrap_or("agb.yaml"),
    };

    let config_path = Path::new(config_file);
    if !config_path.exists() {
        anyhow::bail!("Config file not found: {}", config_file);
    }
    let output_dir = config_path.parent().unwrap_or(Path::new("."));

    println!("Loading config: {}", config_file);
    let cfg = core::load_config(config_file)?;
    let source_dir = Path::new(&cfg.source);

    // Registry 및 Transformer 구축 (Build와 Sync 모두에서 공통으로 필요)
    let registry = loader::load_registry_from_config(&cfg, source_dir)?;
    let transformer = transformer::TransformerFactory::create(&cfg.target);

    match &cli.command {
        Commands::Build { .. } => {
            let executor = Builder::new();
            executor.run(&cfg, transformer.as_ref(), &registry, source_dir, output_dir)?;
        }
        Commands::Sync { .. } => {
            let executor = Syncer::new();
            executor.run(&cfg, transformer.as_ref(), &registry, output_dir)?;
        }
    }

    Ok(())
}
