mod app;
mod cli;
mod constants;
mod handler;
mod models;
mod placeholders;
mod platform;
mod storage;
mod ui;
mod utils;

use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use app::App;
use cli::Cli;
use handler::HandleResult;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = Cli::parse();

    // If CLI flags are present, run headless and exit
    if cli_args.is_headless() {
        return cli::run_headless(cli_args);
    }

    // Otherwise, launch the interactive TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    // Terminal teardown
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    loop {
        app.clamp_selection();

        terminal.draw(|f| ui::draw(f, &mut app))?;

        match handler::handle_events(&mut app)? {
            HandleResult::Quit => return Ok(()),
            HandleResult::Continue => {}
        }
    }
}
