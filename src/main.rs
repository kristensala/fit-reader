use anyhow::Result;

mod db;
mod parser;


//IDEA 1: every time I run the program, check if new workouts are created
//IDEA 2: create a specific command to pull workouts. Makes load up time faster
fn main() -> Result<()> {

    let trainer_road_path = "~/Dropbox/Apps/TrainerRoad/";
    // check config if base file bath is set and db pash is set
    
    // pull workouts

    // iterate over the files and parse them and insert to database
    
    // archive workouts -> move the files to archive folder
    db::init();
//    let session = parser::init()?;
//    db::insert_session(session);

    let test = db::get_all_sessions()?;

    println!("{:?}", test);
    return Ok(());
}

// return list of file paths
fn pull_workouts(dirs: Vec<String>) -> Result<()> {
    // use walkdir crate
    return Ok(()); 
}

// probably delete the file if inserted successfully
fn archive_workouts() -> Result<()> {
    return Ok(());
}

fn is_fit_file(file: String) -> bool {
    if file.ends_with(".fit") {
        return true;
    }

    return false;
}

fn check_ricing() -> Result<()> {
    // ~/.fitreaderrc -> this file needs to exist and certain variables need to be declared in it
    // (folder to watch and the db file location)
    return Ok(());
}
