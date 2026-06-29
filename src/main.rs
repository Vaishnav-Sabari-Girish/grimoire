use anyhow::{Ok, Result};
use clap::Parser;

use grimoire::cli::{Cli, Commands};
use grimoire::config;
use grimoire::executor;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = config::load_grimoire()?;

    match cli.command {
        Commands::Sigils => {
            executor::list_sigils(&config);
        }
        Commands::Cast { name } => {
            executor::cast_sigil(&config, &name).await?;
        }
    }

    Ok(())
}
