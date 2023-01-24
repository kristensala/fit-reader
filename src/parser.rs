use chrono::{NaiveDateTime, DateTime, Utc};
use fitparser::{self, FitDataRecord, FitDataField, Value};
use fitparser::profile::MesgNum;
use core::fmt;
use std::fs::File;
use anyhow::{Result, Context, bail};

use crate::util;

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
    SerialNumber,
    Power,
    Timestamp,
    Distance,
    HeartRate
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: Option<i16>,
    pub start_time: i64,
    pub total_elapsed_time: f64,
    pub total_distance: f64,
    pub avg_power: i64,
    pub total_moving_time: f64,
    pub avg_heart_rate: i64,
    pub threshold_power: i64,
    pub sport: String,
    pub sub_sport: String,
    pub avg_cadence: i64,
    pub laps: Vec<Lap>,
    pub records: Vec<Record>,
    pub serial_num: i64
}

#[derive(Debug, Clone)]
pub struct Lap {
    pub id: Option<i16>,
    pub start_time: i64,
    pub avg_power: i64,
    pub avg_heart_rate: i64,
    pub total_moving_time: f64,
    pub total_distance: f64
}

#[derive(Debug, Clone)]
pub struct Record {
    pub id: Option<i16>,
    pub timestamp: i64,
    pub heart_rate: i64,
    pub power: i64,
    pub distance: f64
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
            FieldName::SerialNumber => write!(f, "serial_number"),
            FieldName::Power => write!(f, "power"),
            FieldName::Timestamp => write!(f, "timestamp"),
            FieldName::Distance => write!(f, "distance"),
            FieldName::HeartRate => write!(f, "heart_rate")
        }
    }
}

impl Session {
    pub fn to_string(&self) -> String {
        let result = format!("{} {} {}"
            , self.timestamp_as_string()
            , self.sub_sport
            , self.moving_time_as_string());
        
        return result;
    }

    pub fn moving_time_as_string(&self) -> String {
        let hours = ((self.total_moving_time / 60.0) / 60.0) as i64;
        let minutes = ((self.total_moving_time / 60.0) % 60.0) as i64;
        let duration = format!("{}h {}m", hours, minutes);
        return duration;
    }
    
    pub fn distance_as_string(&self) -> String {
        return format!("{}km", self.total_distance / 1000.0);
    }

    pub fn timestamp_as_string(&self) -> String {
        let naive_datetime = NaiveDateTime::from_timestamp(self.start_time, 0);
        let start_date_time: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
        let start_date = start_date_time.format("%Y-%m-%d %H:%M:%S").to_string();
        return start_date;
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
            id: None,
            start_time: Value::try_into(start_time_field.value().to_owned()).unwrap(),
            total_elapsed_time: Value::try_into(time_field.value().to_owned()).unwrap(),
            total_distance: Value::try_into(distance_field.value().to_owned()).unwrap(),
            avg_power: Value::try_into(power_field.value().to_owned()).unwrap(),
            total_moving_time: Value::try_into(total_moving_time_field.value().to_owned()).unwrap(),
            avg_heart_rate: Value::try_into(avg_heart_rate_field.value().to_owned()).unwrap(),
            threshold_power: Value::try_into(threshold_power_field.value().to_owned()).unwrap(),
            sport: sport_field.value().to_string(),
            sub_sport: sub_sport_field.value().to_string(),
            avg_cadence: Value::try_into(avg_candence_field.value().to_owned()).unwrap(),
            serial_num: 0,
            laps: Vec::new(),
            records: Vec::new()
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
            id: None,
            start_time: Value::try_into(start_time_field.value().to_owned()).unwrap(),
            total_distance: Value::try_into(distance_field.value().to_owned()).unwrap(),
            avg_power: Value::try_into(power_field.value().to_owned()).unwrap(),
            total_moving_time: Value::try_into(total_moving_time_field.value().to_owned()).unwrap(),
            avg_heart_rate: Value::try_into(avg_heart_rate_field.value().to_owned()).unwrap(),
        }
    }
}

impl <'a>FromIterator<&'a FitDataField> for Record {
    fn from_iter<T: IntoIterator<Item = &'a FitDataField>>(iter: T) -> Record {
        let fields = iter.into_iter()
            .filter(|x| x.name() == FieldName::Power.to_string()
                || x.name() == FieldName::Distance.to_string()
                || x.name() == FieldName::Timestamp.to_string()
                || x.name() == FieldName::HeartRate.to_string())
            .collect::<Vec<&FitDataField>>();

        let power_field = fields.iter()
            .find(|&&x| x.name() == FieldName::Power.to_string());

        let power_option = if power_field.is_some() {
            Value::try_into(power_field.unwrap().value().to_owned()).unwrap()
        } else {
            0 as i64
        };

        let distance_field = fields.iter()
            .find(|&&x| x.name() == FieldName::Distance.to_string());
        
        let distance_option = if distance_field.is_some() { 
            Value::try_into(distance_field.unwrap().value().to_owned()).unwrap()
        } else { 
            0 as f64
        };


        let timestamp_field = fields.iter()
            .find(|&&x| x.name() == FieldName::Timestamp.to_string())
            .unwrap();

        let heart_rate_field = fields.iter()
            .find(|&&x| x.name() == FieldName::HeartRate.to_string());

        let heart_rate_option = if heart_rate_field.is_some() { 
            Value::try_into(heart_rate_field.unwrap().value().to_owned()).unwrap()
        } else { 
            0
        };

        return Record {
            id: None,
            timestamp: Value::try_into(timestamp_field.value().to_owned()).unwrap(),
            distance: distance_option,
            power: power_option,
            heart_rate: heart_rate_option
        }
    }
}

pub fn init(path: &String) -> Result<Session> {
    println!("Parsing FIT files using Profile version: {}", fitparser::profile::VERSION);
    if !is_fit_file(path) {
        bail!("Not a fit file");
    }

    let mut fp = File::open(path)
        .context("Unable to open the file")?;

    let fit_data = fitparser::from_reader(&mut fp)
        .context("Failed to read data from file!")?;

    let session_data: Session = get_session_data(&fit_data)
        .context("Failed getting Session data")?;

    return Ok(session_data);
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
    parsed_data.records = get_record_data(data).unwrap();

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

fn get_record_data(data: &Vec<FitDataRecord>) -> Result<Vec<Record>> {
    let record_data: Vec<&FitDataRecord> = data.into_iter()
        .filter(|x| x.kind() == MesgNum::Record)
        .collect();

    let step = 2;
    let records = record_data.iter()
        .skip(step - 1)
        .step_by(step)
        .map(|&x| {
            let record_fields: Vec<&FitDataField> = x.fields().into_iter().collect();
            return Record::from_iter(record_fields.into_iter());
        }).collect::<Vec<Record>>();

    return Ok(records);
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

fn is_fit_file(file: &String) -> bool {
    if file.ends_with(".fit") {
        return true;
    }

    return false;
}
