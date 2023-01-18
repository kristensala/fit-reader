use std::env;
use std::fs;
use anyhow::Result;
use app::App;

use std::io::stdout;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod ui;
mod db;
mod parser;

fn main() -> Result<()> {
    let mut errors: Vec<String> = Vec::new();
    let mut files_imported: Vec<String> = Vec::new();

    let trainer_road_path = "/home/salakris/Dropbox/Apps/TrainerRoad/";
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "import" {
        db::init()?;

        println!("{}", "Start import");

        for file in fs::read_dir(trainer_road_path).unwrap() {
            let path = file.unwrap().path().display().to_string();
            println!("{}", path);

            let session = parser::init(&path);
            if !session.is_ok() {
                let err = session.err();
                errors.push(format!("Error: {}", err.unwrap()));
                continue;
            }

            let session_insert = db::insert_session(session.unwrap());

            if session_insert.is_ok() {
                files_imported.push(String::from(&path));
                fs::remove_file(path)?;
            } else {
                let error = session_insert.err();
                errors.push(format!("Failed to import session {}; Error: {}", &path, error.unwrap()));
            }
        }

        println!("Errors: {:#?}", errors);
        println!("Files imported: {:#?}", files_imported);
        
        return Ok(());
    }
    
    start_ui()?;

    return Ok(());
}

fn start_ui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let mut app = App::new();
    app.sessions = db::get_all_sessions()?;
    app.selected_session = app.sessions.first().cloned();

    let res = draw(&mut terminal, &app);

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

fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw_dashboard(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

