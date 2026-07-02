use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "grimoire",
    version,
    about = "Make commands feel like spells",
    long_about = "Inspired by Witch Hat Atelier and Frieren Beyond Journey's End"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scribe (Generate) a default Grimoire.toml in the CWD (Current Working directory)
    Init,

    /// Cast a Specific Sigil/Spell (Task) defined in your Grimoire.toml
    #[command(visible_aliases = ["run"])]
    Cast {
        name: String,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
    },

    /// List all available Sigils/Spells (Tasks) in your current spellbook
    #[command(visible_aliases = ["tasks", "list"])]
    Sigils,

    /// List all the tongues (Languages) supported by Grimoire
    /// [alias: tongues]
    #[command(alias = "tongues")]
    Lang,
}
