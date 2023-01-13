use anyhow::Result;
use anyhow::anyhow;
use rusqlite::Connection;

use crate::parser::Session;

//TODO: read a path from ~/.fit-reader file.
//if .fit-reader does not exist or db path variable is missing show and error 
fn open_connection() -> Result<Connection> {
    return Ok(Connection::open("test.db")?);
}

pub fn init() -> Result<()> {
    let connection = open_connection()?;

    connection.execute(
        "create table if not exists session (
            id integer primary key,
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

    connection.execute(
        "create table if not exists lap (
            id integer primary key,
            avg_heart_rate integer null,
            avg_power integer null,
            start_time text not null,
            distance real null,
            total_moving_time real null,
            session_id integer not null,
            foreign key (session_id)
                references session (id)
        )", [])?;

    return Ok(());
}

pub fn insert_session(session: Session) -> Result<i64> {
    let connection = open_connection()?;

    let insert_session = connection.execute(
        "insert into session (sport
            , sub_sport
            , avg_power
            , avg_heart_rate
            , total_distance
            , total_moving_time
            , total_elapsed_time
            , avg_cadence
            , serial_number
            , start_time)
            values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
        , [session.sport
            , session.sub_sport
            , session.avg_power.to_string()
            , session.avg_heart_rate.to_string()
            , session.total_distance.to_string()
            , session.total_moving_time.to_string()
            , session.total_elapsed_time.to_string()
            , session.avg_cadence.to_string()
            , session.serial_num.to_string()
            , session.start_time.to_string()]);

    if insert_session.is_err() {
        return Err(anyhow!("Could not insert session!"));
    }

    let session_id = connection.last_insert_rowid();

    let laps = session.laps;
    for lap in laps {
        connection.execute(
            "insert into lap (
                  avg_heart_rate
                , avg_power
                , start_time
                , distance
                , total_moving_time
                , session_id
            ) values (?1, ?2, ?3, ?4, ?5, ?6)"
            , [lap.avg_heart_rate.to_string()
                , lap.avg_power.to_string()
                , lap.start_time.to_string()
                , lap.total_distance.to_string()
                , lap.total_moving_time.to_string()
                , session_id.to_string()])?;
    }

    return Ok(session_id);
}
