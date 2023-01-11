use rusqlite::{Connection, Result as SqliteResult};

use crate::parser::Session;

fn open_connection() -> SqliteResult<Connection> {
    return Ok(Connection::open("test.db")?);
}

pub fn init() -> SqliteResult<()> {
    let connection = open_connection()?;

    connection.execute(
        "create table if not exists session (
            id integer primary key,
            name text not null
        )",
        []
    )?;

    return Ok(());
}

pub fn insert_session(session: Session) -> SqliteResult<()> {
    let connection = open_connection()?;


    return Ok(());
}

pub fn get_session_by_id(id: i32) -> SqliteResult<()> { // should return session
    return Ok(()); 
}

pub fn get_sessions_by_date_range(start: f64, end: f64) -> SqliteResult<()> {
    return Ok(());
}

