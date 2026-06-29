use anyhow::{Ok, Result};
use clap::Parser;

use grim::cli::{Cli, Commands};
use grim::config;
use grim::executor;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            executor::init_grimoire()?;
        }
        Commands::Sigils => {
            let config = config::load_grimoire()?;
            executor::list_sigils(&config);
        }
        Commands::Cast { name } => {
            let config = config::load_grimoire()?;
            executor::cast_sigil(&config, &name).await?;
        }
    }

    Ok(())
}
