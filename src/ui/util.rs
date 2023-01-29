use chrono::{NaiveDateTime, DateTime, Utc};

use crate::parser::Session;

pub struct ChartDataset {
    pub power: Vec<(f64, f64)> ,
    pub heart_rate: Vec<(f64, f64)> ,
    pub min_y: f64,
    pub max_y: f64,
    pub max_x: f64,
    pub threshold_power: Vec<(f64, f64)> 
}

impl ChartDataset {
    pub fn new(power_data: Vec<(f64, f64)>,
            heart_rate_data: Vec<(f64, f64)>,
            min_y_value: f64,
            max_y_value: f64,
            max_x_value: f64,
            threshold_power_data: Vec<(f64, f64)>) -> Self {
        return Self {
            power: power_data,
            heart_rate: heart_rate_data,
            min_y: min_y_value, 
            max_y: max_y_value,
            max_x: max_x_value,
            threshold_power: threshold_power_data
        }
    }
}

pub fn build_session_dataset(session: &Session) -> ChartDataset {
    let mut power_array: Vec<(f64, f64)> =  Vec::new();
    let mut heart_array: Vec<(f64, f64)> = Vec::new();
    let mut threshold_power_data: Vec<(f64, f64)> = Vec::new();

    for (idx, item) in session.records.iter().enumerate() {
        power_array.push((idx as f64, item.power as f64));
        heart_array.push((idx as f64, item.heart_rate as f64));
        threshold_power_data.push((idx as f64, session.threshold_power as f64));
    }

    let min_value_y = session.records.iter()
        .map(|x| x.heart_rate)
        .collect::<Vec<i64>>()
        .iter()
        .min()
        .unwrap()
        .to_owned() as f64;

    let max_value_y = session.records.iter()
        .map(|x| x.power)
        .collect::<Vec<i64>>()
        .iter()
        .max()
        .unwrap()
        .to_owned() as f64;

    let max_value_x = heart_array.len() as f64;

    let dataset = ChartDataset::new(power_array, heart_array, min_value_y, max_value_y, max_value_x, threshold_power_data);
    return dataset;
}

pub fn sessio_to_string(session: &Session) -> String {
    let result = format!("{} {} {}"
        , timestamp_as_string(session.start_time)
        , session.sub_sport
        , moving_time_to_hour_minute_string(session.total_moving_time));

    return result;
}

pub fn moving_time_to_hour_minute_string(moving_time: f64) -> String {
    let hours = ((moving_time / 60.0) / 60.0) as i64;
    let minutes = ((moving_time / 60.0) % 60.0) as i64;
    let duration = format!("{}h {}m", hours, minutes);
    return duration;
}

pub fn timestamp_as_string(timestamp: i64) -> String {
    let naive_datetime = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    let start_date_time: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
    let start_date = start_date_time.format("%Y-%m-%d %H:%M:%S").to_string();
    return start_date;
}

pub fn distance_as_string(distance: f64) -> String {
    return format!("{:.2}km", distance / 1000.0);
}
