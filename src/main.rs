use std::env;
use std::fs;
use anyhow::Result;

mod app;
mod ui;
mod db;
mod parser;

fn main() -> Result<()> {
    let mut errors: Vec<String> = Vec::new();
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
                fs::remove_file(path)?;
            } else {
                let error = session_insert.err();
                errors.push(format!("Failed to import session {}; Error: {}", &path, error.unwrap()));
            }
        }

        println!("{:#?}", errors);

        return Ok(());
    }
    
    //let sessions = db::get_all_sessions()?;
    //println!("{:#?}", sessions);

    app::run_app()?;
    return Ok(());
}

