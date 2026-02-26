mod builder;
mod core;
mod loader;
mod syncer;
mod transformer;
mod utils;

use builder::BuildExecutor;
use clap::{Parser, Subcommand};
use std::path::Path;

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

    match &cli.command {
        Commands::Build { .. } => {
            let executor = BuildExecutor::new();
            executor.run(&cfg, source_dir, output_dir)?;
        }
        Commands::Sync { .. } => {
            let executor = syncer::SyncExecutor::new();
            executor.run(&cfg, source_dir, output_dir)?;
        }
    }

    Ok(())
}
