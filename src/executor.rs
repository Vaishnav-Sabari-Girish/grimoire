use anyhow::{Context, Result};
use inquire::Select;
use std::collections::HashMap;

use crate::config::{ArgDef, GrimoireConfig};

pub fn list_sigils(config: &GrimoireConfig) {
    println!("Available Sigils in Grimoire v{}:\n", config.version);
    for (name, sigil) in &config.sigils {
        let desc = sigil.description.as_deref().unwrap_or("No Description");
        println!("  {name:<15} - {desc}");
    }
}

pub async fn call_sigil(config: &GrimoireConfig, name: &str) -> Result<()> {
    let sigil = config
        .sigils
        .get(name)
        .with_context(|| format!("Sigil '{}' not found in Grimoire.toml", name))?;

    let mut resolved_args = HashMap::new();

    for (arg_name, arg_def) in &sigil.args {
        match arg_def {
            ArgDef::Simple(val) => {
                resolved_args.insert(arg_name.clone(), val.clone());
            }
            ArgDef::Detailed {
                kind,
                choices,
                default: _,
            } => {
                if kind == "select"
                    && let Some(options) = choices
                {
                    let prompt_msg = format!("Select {}: ", arg_name);
                    let selection = Select::new(&prompt_msg, options.clone()).prompt()?;
                    resolved_args.insert(arg_name.clone(), selection);
                }
            }
        }
    }

    let mut final_run_cmd = sigil.run.clone();
    for (key, val) in resolved_args {
        let template_key = format!("{{{{{}}}}}", key);
        final_run_cmd = final_run_cmd.replace(&template_key, &val);
    }

    println!("> Executing: {}", final_run_cmd);

    Ok(())
}
