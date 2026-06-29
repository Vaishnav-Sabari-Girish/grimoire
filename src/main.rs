use anyhow::{Context, Ok, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

// Spellbook root
#[derive(Debug, Deserialize)]
struct GrimoireConfig {
    version: String,
    // Maps the [sigil.name] blocks into a dictionary
    #[serde(rename = "sigil")]
    sigils: HashMap<String, Sigil>,
}

// The individual spell definitions
#[derive(Debug, Deserialize)]
struct Sigil {
    description: Option<String>,
    language: Option<String>,
    run: String,
    #[serde(default)]
    depends: Vec<String>,
}

fn main() -> Result<()> {
    //1. Look for the file
    let config_content = fs::read_to_string("Grimoire.toml")
        .context("Failed to find or read Grimoire.toml in the current directory")?;

    // Parse the toml into the structs
    let config: GrimoireConfig = toml::from_str(&config_content)
        .context("Failed to parse the Grimoire File. Is the syntax correct ?")?;

    println!("Successfully opened Grimoire v{}", config.version);
    println!("Found {} sigils: \n", config.sigils.len());

    for (name, sigil) in &config.sigils {
        println!("🔮 Sigil: {}", name);

        if let Some(desc) = &sigil.description {
            println!("      Description: {}", desc);
        }

        println!(
            "      Language: {}",
            sigil.language.as_deref().unwrap_or("shell (default)")
        );
        println!("      Run: {}", sigil.run);
        if !sigil.depends.is_empty() {
            println!("      Depends on: {:?}", sigil.depends);
        }
        println!();
    }
    Ok(())
}
