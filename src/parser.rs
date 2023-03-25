use fitparser::{self, FitDataRecord, FitDataField, Value};
use fitparser::profile::MesgNum;
use core::fmt;
use std::fs::File;
use anyhow::{Result, Context, bail};

// todo: alse set equivalent strings like in fieldName
enum SubSport {
    IndoorCycling
}

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

        let sport_field = fields.iter()
            .find(|&&x| x.name() == FieldName::Sport.to_string())
            .unwrap();

        let sub_sport_field = fields.iter()
            .find(|&&x| x.name() == FieldName::SubSport.to_string())
            .unwrap();


        return Session {
            id: None,
            start_time: get_number_value(&fields, FieldName::StartTime),
            total_elapsed_time: get_decimal_value(&fields, FieldName::TotalElapsedTime),
            total_distance: get_decimal_value(&fields, FieldName::TotalDistace),
            avg_power: get_number_value(&fields, FieldName::AvgPower),
            total_moving_time: get_decimal_value(&fields, FieldName::TotalMovingTime),
            avg_heart_rate: get_number_value(&fields, FieldName::AvgHeartRate),
            threshold_power: get_number_value(&fields, FieldName::ThresholdPower),
            sport: sport_field.value().to_string(),
            sub_sport: sub_sport_field.value().to_string(),
            avg_cadence: get_number_value(&fields, FieldName::AvgCadence),
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

        return Lap {
            id: None,
            start_time: get_number_value(&fields, FieldName::StartTime),
            total_distance: get_decimal_value(&fields, FieldName::TotalDistace),
            avg_power: get_number_value(&fields, FieldName::AvgPower),
            total_moving_time: get_decimal_value(&fields, FieldName::TotalMovingTime),
            avg_heart_rate: get_number_value(&fields, FieldName::AvgHeartRate),
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

        return Record {
            id: None,
            timestamp: get_number_value(&fields, FieldName::Timestamp),
            distance: get_decimal_value(&fields, FieldName::Distance),
            power: get_number_value(&fields, FieldName::Power),
            heart_rate: get_number_value(&fields, FieldName::HeartRate)
        }
    }
}

fn get_number_value(fields: &Vec<&FitDataField>, field_name: FieldName) -> i64 {
    let field = fields.iter()
        .find(|&&x| x.name() == field_name.to_string());

    let value = match field {
        Some(data) => Value::try_into(data.value().to_owned()).unwrap_or(0),
        None => 0
    };

    return value;
}

fn get_decimal_value(fields: &Vec<&FitDataField>, field_name: FieldName) -> f64 {
    let field = fields.iter()
        .find(|&&x| x.name() == field_name.to_string());

    let value = match field {
        Some(data) => Value::try_into(data.value().to_owned()).unwrap_or(0.0),
        None => 0.0
    };

    return value;
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

    let records = record_data.iter()
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
