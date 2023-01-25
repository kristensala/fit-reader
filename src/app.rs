use std::io::stdout;

use anyhow::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, self, KeyCode};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen};
use tui::Terminal;
use tui::backend::{CrosstermBackend, Backend};

use crate::parser::Session;
use crate::db;
use crate::ui;
use crate::util;

pub struct App {
    pub sessions: Vec<Session>,
    pub selected_session: Option<Session>,
    pub selected_session_index: Option<usize>
}

impl Default for App {
    fn default() -> Self {
        App {
            selected_session_index: None,
            sessions: Vec::new(),
            selected_session: None
        }
    }
}

impl App {
    pub fn new() -> Self {
        let all_sessions = db::get_all_sessions().unwrap();
        let selected = all_sessions.first().cloned();

        return Self {
            sessions: all_sessions,
            selected_session_index: Some(0),
            selected_session: selected,
            ..App::default()
        }
    }

    pub fn change_selected_session(&mut self, index: usize) {
        self.selected_session = self.sessions.get(index).cloned(); 
    }

    pub fn start_ui(self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = stdout();

        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.draw(&mut terminal);

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

    fn draw<B: Backend>(mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| ui::draw_dashboard(f, &self))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    },
                    KeyCode::Char('j') => {
                        util::move_down_event(&mut self);
                    },
                    KeyCode::Char('k') => {
                        util::move_up_event(&mut self);
                    },
                    KeyCode::Down => {
                        util::move_down_event(&mut self);
                    },
                    KeyCode::Up => {
                        util::move_up_event(&mut self);
                    },
                    _ => ()
                }
            }
        }
    }
}
