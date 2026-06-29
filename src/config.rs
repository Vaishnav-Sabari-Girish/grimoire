use anyhow::{Context, Ok, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct GrimoireConfig {
    pub version: String,
    #[serde(rename = "sigil")]
    pub sigils: HashMap<String, Sigil>,
}

#[derive(Debug, Deserialize)]
pub struct Sigil {
    pub description: Option<String>,
    pub language: Option<String>,
    pub run: String,
    #[serde(default)]
    pub depends: Vec<String>,
    #[serde(default)]
    pub args: HashMap<String, ArgDef>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ArgDef {
    Simple(String),
    Detailed {
        #[serde(rename = "type")]
        kind: String,
        choices: Option<Vec<String>>,
        default: Option<String>,
    },
}

// Attempts to find and load the Grimoire.toml
pub fn load_grimoire() -> Result<GrimoireConfig> {
    let config_content = fs::read_to_string("Grimoire.toml")
        .context("Failed to find or read Grimoire.toml in the current directory")?;

    let config: GrimoireConfig =
        toml::from_str(&config_content).context("Failed to parse the Grimoire file.")?;

    Ok(config)
}
