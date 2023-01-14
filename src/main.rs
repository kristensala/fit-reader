use std::env;
use std::fs;

use anyhow::Result;

mod db;
mod parser;

fn main() -> Result<()> {
    let mut errors: Vec<String> = Vec::new();
    let trainer_road_path = "/home/salakris/Dropbox/Apps/TrainerRoad/";
    let args: Vec<String> = env::args().collect();

    db::init()?;

    if args.len() > 1 && args[1] == "i" {
        println!("{}", "Start import");

        for file in fs::read_dir(trainer_road_path).unwrap() {
            let path = file.unwrap().path().display().to_string();
            println!("{}", path);
            if is_fit_file(&path) {
                let session = parser::init(&path)?;
                let inserted = db::insert_session(session);
                if inserted.is_ok() {
                    fs::remove_file(path)?;
                } else {
                    let error = inserted.err();
                    errors.push(format!("Failed to import session {}; Error: {}", &path, error.unwrap()));
                }
                
            }
        }

        println!("{:#?}", errors);

        return Ok(());
    }
    
    let sessions = db::get_all_sessions()?;

    println!("{:?}", sessions);
    return Ok(());
}

fn is_fit_file(file: &String) -> bool {
    if file.ends_with(".fit") {
        return true;
    }

    return false;
}
