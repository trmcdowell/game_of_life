mod app;
mod ui;

use std::io;

use app::{App, Universe};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::ui;

fn main() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Setup app
    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    // Print error if one occurs
    if let Err(e) = result {
        println!("{e:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        // Main app logic
        if let Event::Key(key) = event::read()? {
            // Skip key releases
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        return Ok(());
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                KeyCode::Char('f') | KeyCode::Char(' ') => app.universe.tick(),
                KeyCode::Char('r') => app.universe = Universe::default(),
                _ => {}
            }
        }
    }
}
