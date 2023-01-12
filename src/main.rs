use anyhow::Result;

pub mod db;
pub mod parser;

fn main() -> Result<()> {
    db::init();
    return parser::init();
}

fn check_ricing() -> Result<()> {
    // ~/.fitreaderrc -> this file needs to exist and certain variables need to be declared in it
    // (folder to watch and the db file location)
    return Ok(());
}
