use anyhow::{Context, Ok, Result, bail};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct RawConfig {
    pub version: String,

    #[serde(default)]
    pub ingredients: HashMap<String, String>,
    #[serde(default)]
    pub global: HashMap<String, String>,

    #[serde(default)]
    pub sigil: HashMap<String, Sigil>,
    #[serde(default)]
    pub task: HashMap<String, Sigil>,
}

#[derive(Debug)]
pub struct GrimoireConfig {
    pub version: String,
    pub ingredients: HashMap<String, String>,
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

    #[serde(default)]
    pub silent: bool,
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

pub fn load_grimoire() -> Result<GrimoireConfig> {
    let config_content = fs::read_to_string("Grimoire.toml")
        .context("Failed to find or read Grimoire.toml in the current directory")?;

    let raw: RawConfig =
        toml::from_str(&config_content).context("Failed to parse the Grimoire file.")?;

    let mut ingredients = raw.ingredients;
    for (key, val) in raw.global {
        if ingredients.contains_key(&key) {
            bail!(
                "Config error: '{}' is defined in both [ingredients] and [global]. Please use only one.",
                key
            );
        }
        ingredients.insert(key, val);
    }

    let mut sigils = raw.sigil;
    for (key, val) in raw.task {
        if sigils.contains_key(&key) {
            bail!(
                "Config error: Ambiguous definition for '{}'. Cannot use both [sigil.{}] and [task.{}].",
                key,
                key,
                key
            );
        }
        sigils.insert(key, val);
    }

    Ok(GrimoireConfig {
        version: raw.version,
        ingredients,
        sigils,
    })
}
