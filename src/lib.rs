pub mod cli;
pub mod config;
pub mod executor;

use anyhow::{Ok, Result};
use clap::Parser;

use cli::{Cli, Commands};

pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            executor::init_grimoire()?;
        }
        Commands::Sigils => {
            let config = config::load_grimoire()?;
            executor::list_sigils(&config);
        }
        Commands::Cast { name, extra_args } => {
            let config = config::load_grimoire()?;
            executor::cast_sigil(&config, &name, extra_args).await?;
        }
    }

    Ok(())
}
