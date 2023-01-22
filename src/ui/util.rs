use crate::{parser::Session, app::ChartDataset};

pub fn build_session_dataset(session: Session) -> ChartDataset {
    let mut power_array: [(f64, f64); 9999] = [(0.0, 0.0); 9999];
    let mut heart_array: [(f64, f64); 9999] = [(0.0, 0.0); 9999];
    let mut threshold_power_data: [(f64, f64); 9999] = [(0.0, 0.0); 9999];

    for (idx, item) in session.records.iter().enumerate() {
        power_array[idx] = (60.0 * idx as f64, item.power as f64);
        heart_array[idx] = (60.0 * idx as f64, item.heart_rate as f64);
        threshold_power_data[idx] = (60.0 * idx as f64, session.threshold_power as f64);
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

    let max_value_x = (session.records.last().unwrap().timestamp - session.records.first().unwrap().timestamp) as f64;

    let dataset = ChartDataset::new(power_array, heart_array, min_value_y, max_value_y, max_value_x, threshold_power_data);
    return dataset;
}

