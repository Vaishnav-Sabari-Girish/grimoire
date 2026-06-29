use anyhow::{Context, Result, bail};
use inquire::Select;
use std::collections::HashMap;
use std::pin::Pin;
use tokio::process::Command;

use crate::config::{ArgDef, GrimoireConfig};

pub fn list_sigils(config: &GrimoireConfig) {
    println!("  Available Sigils in Grimoire v{}:\n", config.version);
    for (name, sigil) in &config.sigils {
        let desc = sigil.description.as_deref().unwrap_or("No description");
        println!("    {name:<15} - {desc}");
    }
}

/// Public entry point for casting a sigil
pub async fn cast_sigil(config: &GrimoireConfig, name: &str) -> Result<()> {
    // We pass an empty path vector to start cycle tracking
    execute_inner(config, name, Vec::new()).await
}

/// Internal asynchronous recursive executor
fn execute_inner<'a>(
    config: &'a GrimoireConfig,
    name: &'a str,
    path: Vec<String>,
) -> Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
    Box::pin(async move {
        // 1. Cycle Detection
        if path.contains(&name.to_string()) {
            bail!("Cyclic dependency detected! Spell fizzled at: {:?}", path);
        }

        let sigil = config
            .sigils
            .get(name)
            .with_context(|| format!("Sigil '{}' not found in the spellbook.", name))?;

        let mut current_path = path.clone();
        current_path.push(name.to_string());

        // 2. Resolve Dependencies (The Fix for Issue 1)
        for dep in &sigil.depends {
            execute_inner(config, dep, current_path.clone()).await?;
        }

        // 3. Resolve Arguments (The Fix for Issue 2)
        let mut resolved_args = HashMap::new();

        for (arg_name, arg_def) in &sigil.args {
            match arg_def {
                ArgDef::Simple(val) => {
                    resolved_args.insert(arg_name.clone(), val.clone());
                }
                ArgDef::Detailed {
                    kind,
                    choices,
                    default,
                } => {
                    if kind == "select" {
                        if let Some(options) = choices {
                            let prompt_msg = format!("Select {}:", arg_name);
                            let mut select = Select::new(&prompt_msg, options.clone());

                            // If a default is provided, pre-select it in the TUI
                            if let Some(def_val) = default
                                && let Some(idx) = options.iter().position(|x| x == def_val)
                            {
                                select = select.with_starting_cursor(idx);
                            }

                            let selection = select.prompt()?;
                            resolved_args.insert(arg_name.clone(), selection);
                        } else if let Some(def_val) = default {
                            // Fallback if select has no choices but has a default
                            resolved_args.insert(arg_name.clone(), def_val.clone());
                        }
                    } else {
                        // Handle other kinds (e.g., "string") by using the default
                        if let Some(def_val) = default {
                            resolved_args.insert(arg_name.clone(), def_val.clone());
                        }
                    }
                }
            }
        }

        // 4. Inject resolved arguments into the run string
        let mut final_run_cmd = sigil.run.clone();
        for (key, val) in resolved_args {
            let template_key = format!("{{{{{}}}}}", key);
            final_run_cmd = final_run_cmd.replace(&template_key, &val);
        }

        println!("> Executing: {}", final_run_cmd);

        // 5. Fully Asynchronous Execution
        let mut child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &final_run_cmd])
                .spawn()
                .context("Failed to spawn Windows command prompt")?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&final_run_cmd)
                .spawn()
                .context("Failed to spawn shell")?
        };

        let status = child.wait().await?;

        if !status.success() {
            bail!("Spell failed: Sigil '{}' exited with {}", name, status);
        }

        Ok(())
    })
}
