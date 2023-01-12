use rusqlite::{Connection, Result as SqliteResult};

use crate::parser::Session;

//TODO: read a path from ~/.fit-reader file.
//if .fit-reader does not exist or db path variable is missing show and error 
fn open_connection() -> SqliteResult<Connection> {
    return Ok(Connection::open("test.db")?);
}

pub fn init() -> SqliteResult<()> {
    let connection = open_connection()?;

    connection.execute(
        "create table if not exists session (
            id integer primary key,
            name text not null,
            sport text not null,
            sub_sport text not null,
            avg_power integer null,
            avg_heart_rate integer null,
            total_distance real null,
            total_moving_time real null,
            total_elapsed_time real not null,
            avg_cadence integer null,
            serial_number integer null,
            start_time text not null
        )",
        []
    )?;

    //todo; create lap table
    //todo: create lap record table
    //todo: create account/user table -> current ftp and maybe something more

    return Ok(());
}

pub fn insert_session(session: Session) -> SqliteResult<()> {
    let connection = open_connection()?;

    connection.execute(
        "insert into session (name
            , sport
            , sub_sport
            , avg_power
            , avg_heart_rate
            , total_distance
            , total_moving_time
            , total_elapsed_time
            , avg_cadence
            , serial_number
            , start_time)
            values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"
        , [String::from("test file name")
            , session.sport
            , session.sub_sport
            , session.avg_power.to_string()
            , session.avg_heart_rate.to_string()
            , session.total_distance.to_string()
            , session.total_moving_time.to_string()
            , session.total_elapsed_time.to_string()
            , session.avg_cadence.to_string()
            , session.serial_num.to_string()
            , session.start_time.to_string()])?;


    return Ok(());
}

pub fn insert_session_in_bulk(sessons: Vec<&Session>) -> SqliteResult<()> {
    return Ok(());
}

pub fn get_session_by_id(id: i32) -> SqliteResult<()> { // should return session
    return Ok(()); 
}

pub fn get_sessions_by_date_range(start: f64, end: f64) -> SqliteResult<()> {
    return Ok(());
}

pub fn get_all_sessions() -> SqliteResult<()>{
    return Ok(());
}

