use fitparser::{self, FitDataRecord, FitDataField, Value};
use fitparser::profile::MesgNum;
use core::fmt;
use std::fs::File;
use anyhow::{Result, Context};

#[derive(Debug)]
enum FieldName {
    StartTime,
    AvgPower,
    TotalMovingTime,
    TotalElapsedTime,
    AvgCadence,
    TotalDistace,
    Sport,
    SubSport,
    ThresholdPower,
    AvgHeartRate,
    SerialNumber
}

// TODO: Session should have a file id also
#[derive(Debug, Clone)]
pub struct Session {
    start_time: i64,
    total_time: f64,
    total_distance: f64,
    avg_power: i64,
    total_moving_time: f64,
    avg_heart_rate: i64,
    threshold_power: i64,
    sport: String,
    sub_sport: String,
    avg_cadence: i64,
    laps:  Vec<Lap>,
    serial_num: i64
}

#[derive(Debug, Clone)]
pub struct Lap {
    start_time: i64,
    avg_power: i64,
    avg_heart_rate: i64,
    total_moving_time: f64,
    total_distance: f64
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
            FieldName::ThresholdPower => write!(f, "threshold_power"),
            FieldName::StartTime => write!(f, "start_time"),
            FieldName::SerialNumber => write!(f, "serial_number")
        }
    }
}

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
                || x.name() == FieldName::StartTime.to_string()
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
            .expect("If the file is not corrupt, heart rate field should exist");

        let start_time_field = fields.iter()
            .find(|&&x| x.name() == FieldName::StartTime.to_string())
            .unwrap();

        return Session {
            start_time: Value::try_into(start_time_field.value().to_owned()).unwrap(),
            total_time: Value::try_into(time_field.value().to_owned()).unwrap(),
            total_distance: Value::try_into(distance_field.value().to_owned()).unwrap(),
            avg_power: Value::try_into(power_field.value().to_owned()).unwrap(),
            total_moving_time: Value::try_into(total_moving_time_field.value().to_owned()).unwrap(),
            avg_heart_rate: Value::try_into(avg_heart_rate_field.value().to_owned()).unwrap(),
            threshold_power: Value::try_into(threshold_power_field.value().to_owned()).unwrap(),
            sport: sport_field.value().to_string(),
            sub_sport: sub_sport_field.value().to_string(),
            avg_cadence: Value::try_into(avg_candence_field.value().to_owned()).unwrap(),
            serial_num: 0,
            laps: Vec::new()
        };
    }
}

impl <'a>FromIterator<&'a FitDataField> for Lap {
    fn from_iter<T: IntoIterator<Item = &'a FitDataField>>(iter: T) -> Lap {
        let fields = iter.into_iter()
            .filter(|x| x.name() == FieldName::AvgPower.to_string()
                || x.name() == FieldName::TotalDistace.to_string()
                || x.name() == FieldName::TotalMovingTime.to_string()
                || x.name() == FieldName::AvgHeartRate.to_string()
                || x.name() == FieldName::StartTime.to_string())
            .collect::<Vec<&FitDataField>>();

        let power_field = fields.iter()
            .find(|&&x| x.name() == FieldName::AvgPower.to_string())
            .unwrap();

        let distance_field = fields.iter()
            .find(|&&x| x.name() == FieldName::TotalDistace.to_string())
            .unwrap();

        let total_moving_time_field = fields.iter()
            .find(|&&x| x.name() == FieldName::TotalMovingTime.to_string())
            .unwrap();

        let avg_heart_rate_field = fields.iter()
            .find(|&&x| x.name() == FieldName::AvgHeartRate.to_string())
            .expect("If the file is not corrupt, heart rate field should exist");

        let start_time_field = fields.iter()
            .find(|&&x| x.name() == FieldName::StartTime.to_string())
            .unwrap();

        return Lap {
            start_time: Value::try_into(start_time_field.value().to_owned()).unwrap(),
            total_distance: Value::try_into(distance_field.value().to_owned()).unwrap(),
            avg_power: Value::try_into(power_field.value().to_owned()).unwrap(),
            total_moving_time: Value::try_into(total_moving_time_field.value().to_owned()).unwrap(),
            avg_heart_rate: Value::try_into(avg_heart_rate_field.value().to_owned()).unwrap(),
        }
    }
}

pub fn init() -> Result<()> {
    println!("Parsing FIT files using Profile version: {}", fitparser::profile::VERSION);
    let mut fp = File::open("/home/salakris/Downloads/salakris-2023-01-08-l2-up-down-150--157110383.fit")
        .context("Unable to open the file")?;

    let fit_data = fitparser::from_reader(&mut fp)
        .context("Failed to read data from file!")?;

    let session_data: Session = get_session_data(&fit_data)
        .context("Failed getting Session data")?;

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

    let mut parsed_data = Session::from_iter(data_fields.into_iter());
    parsed_data.serial_num = get_file_serial_num(data).unwrap();
    parsed_data.laps = get_laps_data(data).unwrap();

    return Ok(parsed_data);
}

fn get_laps_data(data: &Vec<FitDataRecord>) -> Result<Vec<Lap>> {
    let laps_data: Vec<&FitDataRecord> = data.into_iter()
        .filter(|x| x.kind() == MesgNum::Lap)
        .collect();

    let laps = laps_data.iter()
        .map(|&x| {
            let lap_fields: Vec<&FitDataField> = x.fields().into_iter().collect();
            return Lap::from_iter(lap_fields.into_iter());
        }).collect::<Vec<Lap>>();

    return Ok(laps);
}

fn get_lap_record_data() -> Result<()> {
    return Ok(());
}

fn get_file_serial_num(data: &Vec<FitDataRecord>) -> Result<i64> {
    let file_data: Vec<&FitDataRecord> = data.into_iter()
        .filter(|x| x.kind() == MesgNum::FileId)
        .collect();

    let serial_num_value: Value = file_data.iter()
        .map(|x| {
            return x.fields()
                .into_iter()
                .find(|x| x.name() == FieldName::SerialNumber.to_string())
                .unwrap()
                .value()
                .to_owned();
        })
        .collect::<Vec<Value>>()
        .first()
        .unwrap()
        .to_owned();

    let serial_num: i64 = Value::try_into(serial_num_value.to_owned()).unwrap();

    return Ok(serial_num);
}

