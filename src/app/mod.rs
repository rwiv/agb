pub mod cli;
pub mod context;

pub use cli::{Cli, Commands};
pub use context::AppContext;

use crate::builder::Builder;

pub struct App;

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        App
    }

    pub fn run(&self, cli: Cli) -> anyhow::Result<()> {
        let config_file = match &cli.command {
            Commands::Build { config } => config.as_deref().unwrap_or("agb.yaml"),
            Commands::Sync { config } => config.as_deref().unwrap_or("agb.yaml"),
        };

        let ctx = AppContext::init(config_file)?;

        match &cli.command {
            Commands::Build { .. } => self.build(&ctx),
            Commands::Sync { .. } => self.sync(&ctx),
        }
    }

    fn build(&self, ctx: &AppContext) -> anyhow::Result<()> {
        let builder = Builder::new();

        println!("Transforming resources for target: {:?}...", ctx.config.target);

        builder.run(
            ctx.transformer.as_ref(),
            &ctx.registry,
            &ctx.source_dir,
            &ctx.output_dir,
        )?;
        println!("  - Target: {:?}", ctx.config.target);
        println!("  - Resources: {} total", ctx.registry.len());
        Ok(())
    }

    fn sync(&self, ctx: &AppContext) -> anyhow::Result<()> {
        println!(
            "Syncing target changes to source for target: {:?}...",
            ctx.config.target
        );
        let exclude = ctx.config.exclude.clone().unwrap_or_default();

        let syncer = crate::syncer::Syncer::new();
        for res in ctx.registry.all_resources() {
            syncer.sync_resource(res, ctx.transformer.as_ref(), &ctx.output_dir, &exclude)?;
        }

        println!("Sync successful!");
        Ok(())
    }
}
