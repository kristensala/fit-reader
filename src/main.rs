use fitparser::{self, FitDataRecord, FitDataField, Value};
use fitparser::profile::MesgNum;
use core::fmt;
use std::fs::File;
use anyhow::{Result, Context};

pub mod db;

#[derive(Debug)]
enum FieldName {
    AvgPower,
    TotalMovingTime,
    TotalElapsedTime,
    AvgCadence,
    TotalDistace,
    Sport,
    SubSport,
    ThresholdPower,
    AvgHeartRate
}

#[derive(Debug, Clone)]
struct Session {
    total_time: f64,
    total_distance: f64,
    avg_power: i64,
    total_moving_time: f64,
    avg_heart_rate: i64,
    threshold_power: i64,
    sport: String,
    sub_sport: String,
    avg_cadence: i64
}

impl fmt::Display for FieldName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldName::AvgPower => write!(f, "avg_power"),
            FieldName::TotalMovingTime => write!(f, "total_moving_time"),
            FieldName::TotalElapsedTime => write!(f, "total_elapsed_time"),
            FieldName::AvgCadence => write!(f, "avg_cadence"),
            FieldName::TotalDistace => write!(f, "total_distance"),
            FieldName::Sport => write!(f, "sport"),
            FieldName::SubSport => write!(f, "sub_sport"),
            FieldName::AvgHeartRate => write!(f, "avg_heart_rate"),
            FieldName::ThresholdPower => write!(f, "threshold_power")
        }
    }
}

// Looks like I have to do the same for Lap. Maybe could use this one here
// without duplicating too much
impl <'a>FromIterator<&'a FitDataField> for Session {
    fn from_iter<T: IntoIterator<Item = &'a FitDataField>>(iter: T) -> Self {
        let fields = iter.into_iter()
            .filter(|x| x.name() == FieldName::AvgPower.to_string()
                || x.name() == FieldName::TotalDistace.to_string()
                || x.name() == FieldName::TotalElapsedTime.to_string()
                || x.name() == FieldName::TotalMovingTime.to_string()
                || x.name() == FieldName::AvgCadence.to_string()
                || x.name() == FieldName::Sport.to_string()
                || x.name() == FieldName::SubSport.to_string()
                || x.name() == FieldName::AvgHeartRate.to_string()
                || x.name() == FieldName::ThresholdPower.to_string())
            .collect::<Vec<&FitDataField>>();

        let power_field = fields.iter()
            .find(|&&x| x.name() == FieldName::AvgPower.to_string())
            .unwrap();

        let distance_field = fields.iter()
            .find(|&&x| x.name() == FieldName::TotalDistace.to_string())
            .unwrap();

        let time_field = fields.iter()
            .find(|&&x| x.name() == FieldName::TotalElapsedTime.to_string())
            .unwrap();

        let total_moving_time_field = fields.iter()
            .find(|&&x| x.name() == FieldName::TotalMovingTime.to_string())
            .unwrap();

        let avg_candence_field = fields.iter()
            .find(|&&x| x.name() == FieldName::AvgCadence.to_string())
            .unwrap();

        let sport_field = fields.iter()
            .find(|&&x| x.name() == FieldName::Sport.to_string())
            .unwrap();

        let sub_sport_field = fields.iter()
            .find(|&&x| x.name() == FieldName::SubSport.to_string())
            .unwrap();

        let threshold_power_field = fields.iter()
            .find(|&&x| x.name() == FieldName::ThresholdPower.to_string())
            .unwrap();
        
        let avg_heart_rate_field = fields.iter()
            .find(|&&x| x.name() == FieldName::AvgHeartRate.to_string())
            .unwrap();

        return Session {
            total_time: Value::try_into(time_field.value().to_owned()).unwrap(),
            total_distance: Value::try_into(distance_field.value().to_owned()).unwrap(),
            avg_power: Value::try_into(power_field.value().to_owned()).unwrap(),
            total_moving_time: Value::try_into(total_moving_time_field.value().to_owned()).unwrap(),
            avg_heart_rate: Value::try_into(avg_heart_rate_field.value().to_owned()).unwrap(),
            threshold_power: Value::try_into(threshold_power_field.value().to_owned()).unwrap(),
            sport: sport_field.value().to_string(),
            sub_sport: sub_sport_field.value().to_string(),
            avg_cadence: Value::try_into(avg_candence_field.value().to_owned()).unwrap()
        };
    }
}

fn main() -> Result<()> {
    println!("Parsing FIT files using Profile version: {}", fitparser::profile::VERSION);
    let mut fp = File::open("/home/salakris/Downloads/salakris-2023-01-08-l2-up-down-150--157110383.fit")
        .context("Unable to open the file")?;

    let fit_data = fitparser::from_reader(&mut fp)
        .context("Failed to read data from file!")?;

    let session_data: Session = get_session_data(&fit_data)
        .context("Failed getting Session data")?;

    let lap_data = get_lap_data(&fit_data).unwrap();

    println!("{:#?}", session_data);

    return Ok(());
}

fn get_session_data(data: &Vec<FitDataRecord>) -> Result<Session> {
    let session_data: &FitDataRecord = data.into_iter()
        .find(|x| x.kind() == MesgNum::Session)
        .unwrap();

    let data_fields: Vec<&FitDataField> = session_data
        .fields()
        .into_iter()
        .collect();

    let parsed_data = Session::from_iter(data_fields.into_iter());
    return Ok(parsed_data);
}

// lap has same datafields as the session
// I need to put them together somehow
fn get_lap_data(data: &Vec<FitDataRecord>) -> Result<()> {
    let lap_data: Vec<&FitDataRecord> = data.into_iter()
        .filter(|x| x.kind() == MesgNum::Lap)
        .collect();

    println!("{:#?}", lap_data.first().unwrap());
    return Ok(());
}

fn get_lap_record_data() -> Result<()> {
    return Ok(());
}
