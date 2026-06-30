use anyhow::{Context, Result, bail};
use inquire::Select;
use std::fs;
use std::pin::Pin;
use std::{collections::HashMap, path::Path};
use tokio::process::Command;

use crate::config::{ArgDef, GrimoireConfig};

pub fn init_grimoire() -> Result<()> {
    let path = Path::new("Grimoire.toml");

    if path.exists() {
        bail!("A Grimoire.toml already exists");
    }

    let template = r#"version = "1"

[sigil.hello]
description = "A Simple Welcome spell"
language = "shell"
run = "echo 'Welcome to Grimoire!'"
    "#;

    fs::write(path, template).context("Failed to write the new Grimoire.toml file")?;

    println!("Grimoire.toml created. Try running it with 'grim cast hello'");

    Ok(())
}

pub fn list_sigils(config: &GrimoireConfig) {
    println!("Available Sigils in Grimoire v{}:\n", config.version);
    for (name, sigil) in &config.sigils {
        let desc = sigil.description.as_deref().unwrap_or("No description");
        println!("    {name:<15} - {desc}");
    }
}

/// Public entry point for casting a sigil
pub async fn cast_sigil(
    config: &GrimoireConfig,
    name: &str,
    extra_args: Vec<String>,
) -> Result<()> {
    // We pass an empty path vector to start cycle tracking
    execute_inner(config, name, Vec::new(), extra_args).await
}

/// Internal asynchronous recursive executor
fn execute_inner<'a>(
    config: &'a GrimoireConfig,
    name: &'a str,
    path: Vec<String>,
    extra_args: Vec<String>,
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

        // 2. Resolve Dependencies
        for dep in &sigil.depends {
            execute_inner(config, dep, current_path.clone(), Vec::new()).await?;
        }

        // 3. Resolve Arguments
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

        // Notice: We completely removed the `extra_args.join(" ")` logic from here.

        println!("> Executing [{}]: {}", name, final_run_cmd);

        // 5. Fully Asynchronous Execution with Language Routing
        let lang = sigil.language.as_deref().unwrap_or("shell");
        let trailing = extra_args.join(" ");

        let mut cmd = match lang {
            "python" | "python3" => {
                let mut c = Command::new("python3");
                // Pass extra_args as standard process arguments
                c.arg("-c").arg(&final_run_cmd).args(&extra_args);
                c
            }
            "javascript" | "node" => {
                let mut c = Command::new("node");
                // Pass extra_args as standard process arguments
                c.arg("-e").arg(&final_run_cmd).args(&extra_args);
                c
            }
            "c" => {
                let src = ".grimoire_tmp.c";
                let exe = if cfg!(target_os = "windows") {
                    ".grimoire_tmp.exe"
                } else {
                    ".grimoire_tmp"
                };

                std::fs::write(src, &final_run_cmd).expect("Failed to scribe temporary C file");

                if cfg!(target_os = "windows") {
                    let mut c = Command::new("cmd");
                    // Inject trailing args into the execution of the binary
                    let run_str =
                        format!("gcc {src} -o {exe} && {exe} {trailing} & del {src} {exe}");
                    c.args(["/C", &run_str]);
                    c
                } else {
                    let mut c = Command::new("sh");
                    // Inject trailing args into the execution of the binary
                    let run_str =
                        format!("gcc {src} -o {exe} && ./{exe} {trailing}; rm -f {src} {exe}");
                    c.arg("-c").arg(&run_str);
                    c
                }
            }
            "cpp" | "c++" => {
                let src = ".grimoire_tmp.cpp";
                let exe = if cfg!(target_os = "windows") {
                    ".grimoire_tmp.exe"
                } else {
                    ".grimoire_tmp"
                };

                std::fs::write(src, &final_run_cmd).expect("Failed to scribe temporary C++ file");

                if cfg!(target_os = "windows") {
                    let mut c = Command::new("cmd");
                    let run_str =
                        format!("g++ {src} -o {exe} && {exe} {trailing} & del {src} {exe}");
                    c.args(["/C", &run_str]);
                    c
                } else {
                    let mut c = Command::new("sh");
                    let run_str =
                        format!("g++ {src} -o {exe} && ./{exe} {trailing}; rm -f {src} {exe}");
                    c.arg("-c").arg(&run_str);
                    c
                }
            }
            "bash" => {
                let run_str = if trailing.is_empty() {
                    final_run_cmd.clone()
                } else {
                    format!("{} {}", final_run_cmd, trailing)
                };
                let mut c = Command::new("bash");
                c.arg("-c").arg(&run_str);
                c
            }
            "zsh" => {
                let run_str = if trailing.is_empty() {
                    final_run_cmd.clone()
                } else {
                    format!("{} {}", final_run_cmd, trailing)
                };
                let mut c = Command::new("zsh");
                c.arg("-c").arg(&run_str);
                c
            }
            "powershell" | "pwsh" => {
                let run_str = if trailing.is_empty() {
                    final_run_cmd.clone()
                } else {
                    format!("{} {}", final_run_cmd, trailing)
                };
                let mut c = Command::new("pwsh");
                c.arg("-Command").arg(&run_str);
                c
            }
            // Fallback for "shell", "sh", or unrecognized languages
            _ => {
                let run_str = if trailing.is_empty() {
                    final_run_cmd.clone()
                } else {
                    format!("{} {}", final_run_cmd, trailing)
                };
                if cfg!(target_os = "windows") {
                    let mut c = Command::new("cmd");
                    c.args(["/C", &run_str]);
                    c
                } else {
                    let mut c = Command::new("sh");
                    c.arg("-c").arg(&run_str);
                    c
                }
            }
        };

        let mut child = cmd
            .spawn()
            .with_context(|| format!("Failed to spawn interpreter for language '{}'", lang))?;

        let status = child.wait().await?;

        if !status.success() {
            bail!("Spell failed: Sigil '{}' exited with {}", name, status);
        }

        Ok(())
    })
}
