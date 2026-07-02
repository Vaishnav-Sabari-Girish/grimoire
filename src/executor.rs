use anyhow::{Context, Result, bail};
use inquire::Select;
use std::fs;
use std::io::{BufRead, BufReader};
use std::pin::Pin;
use std::{collections::HashMap, path::Path};
use tokio::process::Command;

use crate::config::{ArgDef, GrimoireConfig};

fn detect_language_from_file(path: &Path) -> Option<String> {
    if let Ok(file) = std::fs::File::open(path) {
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();

        if reader.read_line(&mut first_line).is_ok() && first_line.starts_with("#!") {
            let parts: Vec<&str> = first_line[2..].split_whitespace().collect();
            if let Some(last) = parts.last() {
                let interpreter = last.split('/').next_back().unwrap_or("");
                if !interpreter.is_empty() {
                    return Some(interpreter.to_string());
                }
            }
        }
    }
    None
}

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

pub async fn cast_sigil(
    config: &GrimoireConfig,
    name: &str,
    extra_args: Vec<String>,
) -> Result<()> {
    execute_inner(config, name, Vec::new(), extra_args).await
}

fn execute_inner<'a>(
    config: &'a GrimoireConfig,
    name: &'a str,
    path: Vec<String>,
    extra_args: Vec<String>,
) -> Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
    Box::pin(async move {
        if path.contains(&name.to_string()) {
            bail!("Cyclic dependency detected! Spell fizzled at: {:?}", path);
        }

        let sigil = config
            .sigils
            .get(name)
            .with_context(|| format!("Sigil '{}' not found in the spellbook.", name))?;

        let mut current_path = path.clone();
        current_path.push(name.to_string());

        for dep in &sigil.depends {
            execute_inner(config, dep, current_path.clone(), Vec::new()).await?;
        }

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

                            if let Some(def_val) = default
                                && let Some(idx) = options.iter().position(|x| x == def_val)
                            {
                                select = select.with_starting_cursor(idx);
                            }

                            let selection = select.prompt()?;
                            resolved_args.insert(arg_name.clone(), selection);
                        } else if let Some(def_val) = default {
                            resolved_args.insert(arg_name.clone(), def_val.clone());
                        }
                    } else {
                        if let Some(def_val) = default {
                            resolved_args.insert(arg_name.clone(), def_val.clone());
                        }
                    }
                }
            }
        }

        let mut final_run_cmd = sigil.run.clone();
        let mut merged_args = HashMap::new();

        for (key, val) in &config.ingredients {
            merged_args.insert(key.clone(), val.clone());
        }

        for (key, val) in resolved_args {
            merged_args.insert(key, val);
        }

        for (key, val) in merged_args {
            let template_key = format!("{{{{{}}}}}", key);
            final_run_cmd = final_run_cmd.replace(&template_key, &val);
        }

        if !sigil.silent {
            println!("> Executing [{}]: {}", name, final_run_cmd);
        }

        let path = Path::new(&final_run_cmd);
        let is_file = path.is_file();

        let mut lang_opt = sigil.language.clone();

        if is_file && lang_opt.is_none() && cfg!(target_os = "windows") {
            lang_opt = detect_language_from_file(path);
        }

        let lang = lang_opt.as_deref().unwrap_or("shell");
        let trailing = extra_args.join(" ");

        let mut temp_files_to_cleanup = Vec::new();

        let mut cmd = match lang {
            "python" | "python3" => {
                let mut c = Command::new("python3");
                if is_file {
                    c.arg(&final_run_cmd).args(&extra_args);
                } else {
                    c.arg("-c").arg(&final_run_cmd).args(&extra_args);
                }
                c
            }
            "javascript" | "node" => {
                let mut c = Command::new("node");
                if is_file {
                    c.arg(&final_run_cmd).args(&extra_args);
                } else {
                    c.arg("-e").arg(&final_run_cmd).args(&extra_args);
                }
                c
            }
            "c" => {
                let src = if is_file {
                    final_run_cmd.clone()
                } else {
                    ".grimoire_tmp.c".to_string()
                };
                let exe = if cfg!(target_os = "windows") {
                    ".grimoire_tmp.exe"
                } else {
                    ".grimoire_tmp"
                };
                let exe_run = if cfg!(target_os = "windows") {
                    ".\\.grimoire_tmp.exe"
                } else {
                    "./.grimoire_tmp"
                };

                if !is_file {
                    std::fs::write(&src, &final_run_cmd)
                        .expect("Failed to scribe temporary C file");
                    temp_files_to_cleanup.push(src.clone());
                }

                temp_files_to_cleanup.push(exe.to_string());

                let compile_status = Command::new("gcc")
                    .args([&src, "-o", exe])
                    .status()
                    .await
                    .context("Failed to execute gcc. Is it installed?")?;

                if !compile_status.success() {
                    if !is_file {
                        std::fs::remove_file(&src).ok();
                    }
                    std::fs::remove_file(exe).ok();
                    bail!("C compilation failed for sigil '{}'", name);
                }

                let mut c = Command::new(exe_run);
                c.args(&extra_args);
                c
            }
            "cpp" | "c++" => {
                let src = if is_file {
                    final_run_cmd.clone()
                } else {
                    ".grimoire_tmp.cpp".to_string()
                };
                let exe = if cfg!(target_os = "windows") {
                    ".grimoire_tmp.exe"
                } else {
                    ".grimoire_tmp"
                };
                let exe_run = if cfg!(target_os = "windows") {
                    ".\\.grimoire_tmp.exe"
                } else {
                    "./.grimoire_tmp"
                };

                if !is_file {
                    std::fs::write(&src, &final_run_cmd)
                        .expect("Failed to scribe temporary C++ file");
                    temp_files_to_cleanup.push(src.clone());
                }

                temp_files_to_cleanup.push(exe.to_string());

                let compile_status = Command::new("g++")
                    .args([&src, "-o", exe])
                    .status()
                    .await
                    .context("Failed to execute g++. Is it installed?")?;

                if !compile_status.success() {
                    if !is_file {
                        std::fs::remove_file(&src).ok();
                    }
                    std::fs::remove_file(exe).ok();
                    bail!("C++ compilation failed for sigil '{}'", name);
                }

                let mut c = Command::new(exe_run);
                c.args(&extra_args);
                c
            }
            "bash" => {
                let mut c = Command::new("bash");
                if is_file {
                    c.arg(&final_run_cmd).args(&extra_args);
                } else {
                    let run_str = if trailing.is_empty() {
                        final_run_cmd.clone()
                    } else {
                        format!("{} {}", final_run_cmd, trailing)
                    };
                    c.arg("-c").arg(&run_str);
                }
                c
            }
            "zsh" => {
                let mut c = Command::new("zsh");
                if is_file {
                    c.arg(&final_run_cmd).args(&extra_args);
                } else {
                    let run_str = if trailing.is_empty() {
                        final_run_cmd.clone()
                    } else {
                        format!("{} {}", final_run_cmd, trailing)
                    };
                    c.arg("-c").arg(&run_str);
                }
                c
            }
            "powershell" | "pwsh" => {
                let mut c = Command::new("pwsh");
                if is_file {
                    c.arg("-File").arg(&final_run_cmd).args(&extra_args);
                } else {
                    let run_str = if trailing.is_empty() {
                        final_run_cmd.clone()
                    } else {
                        format!("{} {}", final_run_cmd, trailing)
                    };
                    c.arg("-Command").arg(&run_str);
                }
                c
            }
            _ => {
                if is_file {
                    let mut c = Command::new(&final_run_cmd);
                    c.args(&extra_args);
                    c
                } else {
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
            }
        };

        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                bail!(
                    "Spell failed: Permission denied. Ensure '{}' has execution rights (e.g., chmod +x).",
                    final_run_cmd
                );
            }
            Err(e) => {
                return Err(anyhow::Error::new(e).context(format!(
                    "Failed to spawn interpreter for language '{}'",
                    lang
                )));
            }
        };

        let status = child.wait().await?;

        for file in temp_files_to_cleanup {
            std::fs::remove_file(file).ok();
        }

        if !status.success() {
            bail!("Spell failed: Sigil '{}' exited with {}", name, status);
        }

        Ok(())
    })
}
