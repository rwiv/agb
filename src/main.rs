use agb::app::{App, Cli};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    let app = App::new();
    app.run(cli)
}
