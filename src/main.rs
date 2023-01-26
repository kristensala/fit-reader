use std::env;
use std::fs;
use anyhow::Result;
use app::App;

mod app;
mod ui;
mod db;
mod parser;
mod util;
mod summary;

fn main() -> Result<()> {
    let mut errors: Vec<String> = Vec::new();
    let mut files_imported: Vec<String> = Vec::new();

    let trainer_road_path = "/home/salakris/Dropbox/Apps/TrainerRoad/";
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "import" {
        // TODO: add import as MTB, road or indoor_cycling parameters
        db::create()?;

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

            let session_insert_response = db::insert_session(session.unwrap());

            if session_insert_response.is_ok() {
                files_imported.push(String::from(&path));
                fs::remove_file(path)?;
            } else {
                let error = session_insert_response.err();
                errors.push(format!("Failed to import session {}; Error: {}", &path, error.unwrap()));
            }
        }

        println!("Errors: {:#?}", errors);
        println!("Files imported: {:#?}", files_imported);
        
        return Ok(());
    }

    let app = App::new();
    app.start_ui()?;

    return Ok(());
}

