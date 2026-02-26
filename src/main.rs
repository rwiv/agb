mod builder;
mod core;
mod loader;
mod syncer;
mod transformer;
mod utils;

use builder::BuildExecutor;
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

    match &cli.command {
        Commands::Build { config } => {
            let config_file = config.as_deref().unwrap_or("agb.yaml");
            let executor = BuildExecutor::new(config_file);
            executor.run()?;
        }
        Commands::Sync { config } => {
            let config_file = config.as_deref().unwrap_or("agb.yaml");
            let executor = syncer::SyncExecutor::new(config_file);
            executor.run()?;
        }
    }

    Ok(())
}
