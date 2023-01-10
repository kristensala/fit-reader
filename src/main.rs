use fitparser::{self, FitDataRecord, FitDataField, Value};
use fitparser::profile::MesgNum;
use core::fmt;
use std::fs::File;
use anyhow::Result;

pub mod db;

#[derive(Debug)]
enum FieldName {
    AvgPower,
    TotalRideTime,
    TotalElapsedTime,
    AvgCadence,
    TotalDistace
}

#[derive(Debug, Clone)]
struct Session {
    total_time: f64,
    total_distance: f64,
    avg_power: f64,
    total_moving_time: f64,
    avg_heart_rate: f64,
    threshold_power: f64
}

impl fmt::Display for FieldName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldName::AvgPower => write!(f, "avg_power"),
            FieldName::TotalRideTime => write!(f, "total_moving_time"),
            FieldName::TotalElapsedTime => write!(f, "total_elapsed_time"),
            FieldName::AvgCadence => write!(f, "avg_cadence"),
            FieldName::TotalDistace => write!(f, "total_distance")
        }
    }
}

impl <'a>FromIterator<&'a FitDataField> for Session {
    fn from_iter<T: IntoIterator<Item = &'a FitDataField>>(iter: T) -> Self {
        let fields = iter.into_iter()
            .filter(|x| x.name() == FieldName::AvgPower.to_string()
                || x.name() == FieldName::TotalDistace.to_string()
                || x.name() == FieldName::TotalElapsedTime.to_string())
            .collect::<Vec<&FitDataField>>();

        let power = fields.iter()
            .find(|&&x| x.name() == FieldName::AvgPower.to_string())
            .unwrap();

        let distance = fields.iter()
            .find(|&&x| x.name() == FieldName::TotalDistace.to_string())
            .unwrap();

        let time = fields.iter()
            .find(|&&x| x.name() == FieldName::TotalElapsedTime.to_string())
            .unwrap();

        return Session {
            total_time: Value::try_into(time.value().to_owned()).unwrap(),
            total_distance: Value::try_into(distance.value().to_owned()).unwrap(),
            avg_power: Value::try_into(power.value().to_owned()).unwrap(),
            total_moving_time: 0.0,
            avg_heart_rate: 0.0,
            threshold_power: 0.0
        };
    }
}

fn main() {
    println!("Parsing FIT files using Profile version: {}", fitparser::profile::VERSION);
    let mut fp = File::open("/home/salakris/Downloads/salakris-2023-01-08-l2-up-down-150--157110383.fit").unwrap();
    let fit_data = fitparser::from_reader(&mut fp).unwrap();
    let session_data: Session = get_session_data(&fit_data).unwrap();
    println!("{:#?}", fit_data);
}

fn get_session_data(data: &Vec<FitDataRecord>) -> Result<Session> {
    let session_data: Vec<&FitDataRecord> = data.into_iter()
        .filter(|x| x.kind() == MesgNum::Session)
        .collect();

    let data_fields: Vec<&FitDataField> = session_data.first().unwrap().fields().into_iter().collect();
    let parsed_data = Session::from_iter(data_fields.into_iter());
    return Ok(parsed_data);
}

fn get_lap_data(data: Vec<FitDataRecord>) -> Result<()> {

    return Ok(());
}

fn get_lap_record_data() -> Result<()> {
    return Ok(());
}
