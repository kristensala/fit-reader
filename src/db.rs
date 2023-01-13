use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use rusqlite::Connection;
use rusqlite::OptionalExtension;

use crate::parser::Session;
use crate::parser::Lap;

//TODO: read a path from ~/.fit-reader file.
//if .fit-reader does not exist or db path variable is missing show and error 
fn open_connection() -> Result<Connection> {
    let conn = match Connection::open("test.db") {
        Ok(connection) => connection,
        Err(e) => bail!(e),
    };
    return Ok(conn);
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
            start_time text not null,
            threshold_power integer null
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

    connection.execute(
        "create table if not exists record (
            id integer primary key,
            heart_rate integer null,
            power integer null,
            timestamp text not null,
            distance real null,
            session_id integer not null,
            foreign key (session_id)
                references session (id)
        )", [])?;

    return Ok(());
}

pub fn insert_session(session: Session) -> Result<i64> {
    if session_exists(session.clone())? {
        return Err(anyhow!("Session already exists during this time period!"));
    }

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
            , start_time
            , threshold_power)
            values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"
        , [session.sport
            , session.sub_sport
            , session.avg_power.to_string()
            , session.avg_heart_rate.to_string()
            , session.total_distance.to_string()
            , session.total_moving_time.to_string()
            , session.total_elapsed_time.to_string()
            , session.avg_cadence.to_string()
            , session.serial_num.to_string()
            , session.start_time.to_string()
            , session.threshold_power.to_string()]);

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

    let records = session.records;
    for record in records {
        connection.execute(
            "insert into record (
                  heart_rate
                , power
                , timestamp
                , distance
                , session_id
            ) values (?1, ?2, ?3, ?4, ?5)"
            , [record.heart_rate.to_string()
                , record.power.to_string()
                , record.timestamp.to_string()
                , record.distance.to_string()
                , session_id.to_string()])?;
    }

    return Ok(session_id);
}

pub fn get_all_sessions() -> Result<Vec<Session>> {
    let conn = open_connection()?;

    let mut query = conn.prepare(
        "select id 
            , sport
            , sub_sport
            , avg_power
            , avg_heart_rate
            , total_distance
            , total_moving_time
            , total_elapsed_time
            , avg_cadence
            , serial_number
            , start_time
            , threshold_power
        from session")?;

    let query_result = query.query_map([], |row| {
        let session_id: i16 = row.get(0)?;
        let sport_col: String = row.get(1)?;
        let sub_sport_col: String = row.get(2)?;
        let avg_power_col: i64 = row.get(3)?;
        let avg_heart_rate_col: i64 = row.get(4)?;
        let total_distance_col: f64 = row.get(5)?;
        let total_moving_time_col: f64 = row.get(6)?;
        let total_elapsed_time_col: f64 = row.get(7)?;
        let avg_cadence_col: i64 = row.get(8)?;
        let serial_num_col: i64 = row.get(9)?;
        let start_time_col: String = row.get(10)?;
        let threshold_power_col: i64 = row.get(11)?;

        Ok(Session {
            id: Some(session_id),
            sport: sport_col,
            sub_sport: sub_sport_col,
            avg_power: avg_power_col,
            avg_heart_rate: avg_heart_rate_col,
            total_distance: total_distance_col,
            total_moving_time: total_moving_time_col,
            total_elapsed_time: total_elapsed_time_col,
            avg_cadence: avg_cadence_col,
            serial_num: serial_num_col,
            start_time: start_time_col.parse::<i64>().unwrap(),
            threshold_power: threshold_power_col,
            laps: Vec::new(),
            records: Vec::new()
        })
    })?;

    let sessions: Vec<Session> = query_result.into_iter()
        .map(|x| x.unwrap())
        .collect();

    return Ok(sessions);
}

fn get_laps_by_session_id(session_id: String) -> Result<Vec<Lap>> {
    let conn = open_connection()?;

    let mut query = conn.prepare(
        "select id 
            , avg_heart_rate
            , avg_power
            , start_time
            , distance
            , total_moving_time
        from lap
        where session_id = ?")?;

    let query_result = query.query_map([session_id], |row| {
        Ok(Lap {
            id: row.get(0).optional()?,
            avg_heart_rate: row.get(1)?,
            avg_power: row.get(2)?,
            start_time: row.get(3)?,
            total_distance: row.get(4)?,
            total_moving_time: row.get(5)?
        })
    })?;

    let laps: Vec<Lap> = query_result.into_iter()
        .map(|x| x.unwrap())
        .collect();

    return Ok(laps);
}

pub fn get_sessions_by_year(year: i64) -> Result<Vec<Session>> {
    let result: Vec<Session> = Vec::new();
    return Ok(result);
}

fn session_exists(session: Session) -> Result<bool> {
    let conn = open_connection()?;
    let start_time = session.start_time;
    let end_time = session.total_elapsed_time.to_string().parse::<i64>().unwrap() + start_time;

    let result = conn.execute(
        "select s.id
              , s.start_time + s.total_elapsed_time as end_time
        from session s
        where (?1 between s.start_time and end_time or ?2 between s.start_time and end_time)
            or (?1 <= s.start_time && ?2 >= endtime)", [start_time, end_time])?;

    if result > 0 {
        return Ok(true);
    }

    return Ok(false);
}
