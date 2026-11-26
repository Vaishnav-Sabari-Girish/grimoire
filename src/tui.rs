use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use std::io::Write;

use crate::config;
use crate::executor;

fn osc9_4_set_indeterminate() {
    let _ = write!(std::io::stdout(), "\x1b]9;4;3;0\x07");
    let _ = std::io::stdout().flush();
}

fn osc9_4_clear() {
    let _ = write!(std::io::stdout(), "\x1b]9;4;0;0\x07");
    let _ = std::io::stdout().flush();
}

fn osc9_4_error() {
    let _ = write!(std::io::stdout(), "\x1b]9;4;2;0\x07");
    let _ = std::io::stdout().flush();
}

pub async fn run_tui() -> Result<()> {
    let cfg = config::load_grimoire()?;

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &cfg).await;

    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
    let _ = terminal.show_cursor();

    result
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    cfg: &config::GrimoireConfig,
) -> Result<()> {
    let names: Vec<String> = cfg.sigils.keys().cloned().collect();
    let mut list_state = ListState::default();
    if !names.is_empty() {
        list_state.select(Some(0));
    }
    let mut last_run_status: Option<String> = None;

    loop {
        terminal.draw(|f| ui(f, cfg, &names, &mut list_state, last_run_status.as_deref()))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Down | KeyCode::Char('j') => {
                    if names.is_empty() {
                        continue;
                    }
                    let i = match list_state.selected() {
                        Some(i) => {
                            if i >= names.len() - 1 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    list_state.select(Some(i));
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if names.is_empty() {
                        continue;
                    }
                    let i = match list_state.selected() {
                        Some(i) => {
                            if i == 0 {
                                names.len() - 1
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    list_state.select(Some(i));
                }
                KeyCode::Enter => {
                    if let Some(i) = list_state.selected()
                        && let Some(name) = names.get(i)
                    {
                        let _ = disable_raw_mode();
                        let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
                        let _ = terminal.show_cursor();

                        osc9_4_set_indeterminate();
                        let cast_res = executor::cast_sigil(cfg, name, vec![]).await;
                        match &cast_res {
                            Ok(_) => osc9_4_clear(),
                            Err(_) => osc9_4_error(),
                        }

                        last_run_status = match cast_res {
                            Ok(_) => Some(format!(
                                "Last run: '{}' succeeded. Press q/Esc to quit.",
                                name
                            )),
                            Err(e) => Some(format!(
                                "Last run: '{}' failed: {:#}. Press q/Esc to quit.",
                                name, e
                            )),
                        };

                        let _ = enable_raw_mode();
                        let mut stdout = std::io::stdout();
                        let _ = execute!(stdout, EnterAlternateScreen);
                        let backend = CrosstermBackend::new(stdout);
                        *terminal = Terminal::new(backend)?;
                    }
                }
                _ => {}
            }
        }
    }
}

fn ui(
    f: &mut Frame,
    cfg: &config::GrimoireConfig,
    names: &[String],
    list_state: &mut ListState,
    last_run_status: Option<&str>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(5),
                Constraint::Length(8),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    let items: Vec<ListItem> = names
        .iter()
        .map(|n| {
            let desc = cfg
                .sigils
                .get(n)
                .and_then(|s| s.description.as_deref())
                .unwrap_or("No description");
            ListItem::new(format!("{:<20} {}", n, desc))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Grimoire Sigils"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(list, chunks[0], list_state);

    let preview_text = if let Some(i) = list_state.selected()
        && let Some(name) = names.get(i)
        && let Some(sigil) = cfg.sigils.get(name)
    {
        let desc = sigil.description.as_deref().unwrap_or("No description");
        let lang = sigil.language.as_deref().unwrap_or("(auto/shell)");
        let deps = if sigil.depends.is_empty() {
            "none".to_string()
        } else {
            sigil.depends.join(", ")
        };
        let run_preview = if sigil.run.len() > 200 {
            format!("{}...", &sigil.run[..200])
        } else {
            sigil.run.clone()
        };
        format!(
            "Name: {}\nDescription: {}\nLanguage: {}\nDepends: {}\nRun:\n{}",
            name, desc, lang, deps, run_preview
        )
    } else {
        "No sigil selected".to_string()
    };
    let preview = Paragraph::new(preview_text)
        .block(Block::default().borders(Borders::ALL).title("Preview"))
        .wrap(Wrap { trim: true });
    f.render_widget(preview, chunks[1]);

    let help_text = if let Some(status) = last_run_status {
        status.to_string()
    } else {
        "Up/Down or j/k to move, Enter to cast, q/Esc to quit".to_string()
    };
    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Status/Help"))
        .wrap(Wrap { trim: true });
    f.render_widget(help, chunks[2]);
}
