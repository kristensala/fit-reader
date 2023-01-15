use crate::{ui, db, parser::Session};

use std::io::stdout;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub struct App {
    pub latest_session: Option<Session>,
}

impl Default for App {
    fn default() -> Self {
        App {
            latest_session: None,
        }
    }
}

pub fn run_app() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = draw(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

/// needs a refactor, just testing right now
fn draw<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut app = App::default();
    
    let sessions = db::get_all_sessions()?;
    let first = sessions.get(0).unwrap();

    app.latest_session = Some(first.to_owned());

    loop {
        terminal.draw(|f| ui::draw_dashboard(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}
