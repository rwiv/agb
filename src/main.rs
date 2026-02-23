mod config;

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
            println!("Loading config: {}", config);
            let cfg = config::load_config(config)?;
            println!("Successfully loaded config: {:?}", cfg);
            // TODO: Implement build logic in Phase 4
        }
    }

    Ok(())
}
