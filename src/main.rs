mod infrastructure;
mod ui;

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use infrastructure::config::AppConfig;
use infrastructure::lm_studio::ModelInfo;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

#[derive(Parser, Debug)]
#[command(
    name = "lms-claude",
    about = "Native Rust TUI launcher for Claude Code using LM Studio"
)]
struct Cli {
    #[arg(short, long)]
    model: Option<String>,

    #[arg(long)]
    check: bool,

    #[arg(long)]
    list: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let hardcoded_default = "openai/gpt-oss-20b".to_string();

    if args.check {
        println!("Checando ambiente...");
        return Ok(());
    }

    if args.list {
        for m in infrastructure::lm_studio::list_models()? {
            println!("{}", m.id);
        }
        return Ok(());
    }

    let mut app_config = AppConfig::load();

    // Precedência: CLI > Config Salva > Default
    let initial_model = args
        .model
        .or_else(|| app_config.selected_model.clone())
        .unwrap_or(hardcoded_default);

    let mut models = infrastructure::lm_studio::list_models().unwrap_or_default();
    if models.is_empty() {
        models.push(ModelInfo {
            id: initial_model.clone(),
        });
    }

    let mut model_to_use = initial_model;
    let mut selected_index = models
        .iter()
        .position(|m| m.id == model_to_use)
        .unwrap_or(0);
    model_to_use = models[selected_index].id.clone();

    infrastructure::lm_studio::start_server()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    while event::poll(std::time::Duration::from_millis(0))? {
        let _ = event::read()?;
    }
    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            ui::screens::dashboard::render(f, &model_to_use, &models, selected_index);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Esc => break,
                KeyCode::Up => {
                    if !models.is_empty() {
                        selected_index = if selected_index == 0 {
                            models.len() - 1
                        } else {
                            selected_index - 1
                        };
                    }
                }
                KeyCode::Down => {
                    if !models.is_empty() {
                        selected_index = (selected_index + 1) % models.len();
                    }
                }
                KeyCode::Char(' ') => {
                    if !models.is_empty() {
                        model_to_use = models[selected_index].id.clone();
                        app_config.selected_model = Some(model_to_use.clone());
                        let _ = app_config.save(); // Salva silenciosamente
                    }
                }
                KeyCode::Enter => {
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;

                    infrastructure::claude_code::launch_claude(&model_to_use)?;
                    return Ok(());
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
