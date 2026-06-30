use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "grimoire",
    version = "0.1.0",
    about = "Make commands feel like spells",
    long_about = "Inspired by Witch Hat Atelier and Frieren Beyond Journey's End"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    Cast {
        name: String,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
    },
    Sigils,
}
