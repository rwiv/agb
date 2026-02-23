mod config;
mod core;
mod transformers;

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

            // Phase 2: Core Loader & Registry
            let plugins_dir = "tests/fixtures/plugins";
            let exclude = cfg.exclude.unwrap_or_default();
            
            // 1. 모든 플러그인 파일 스캔
            let files = core::loader::scan_plugins(plugins_dir, &exclude)?;
            
            // 2. 리소스 객체로 로드
            let all_resources = core::loader::load_resources(plugins_dir, files)?;
            
            // 3. Registry 구축 및 agb.yaml에 명시된 리소스 필터링
            let mut registry = core::registry::Registry::new();
            
            // agb.yaml에서 명시된 리소스 식별자들을 수집 (예: "plugin_a:foo")
            let mut target_identifiers = std::collections::HashSet::new();
            if let Some(cmds) = &cfg.resources.commands { target_identifiers.extend(cmds); }
            if let Some(agents) = &cfg.resources.agents { target_identifiers.extend(agents); }
            if let Some(skills) = &cfg.resources.skills { target_identifiers.extend(skills); }

            for res in all_resources {
                let identifier = format!("{}:{}", res.plugin(), res.name());
                if target_identifiers.contains(&identifier) {
                    registry.register(res)?;
                }
            }

            println!("Successfully initialized registry with {} resources.", registry.len());
            for res in registry.all_resources() {
                println!("  - Registered: {} (from {})", res.name(), res.plugin());
            }
        }
    }

    Ok(())
}
