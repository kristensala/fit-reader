use crate::app::App;

pub struct ChartDataset {
    pub power: [(f64, f64); 200],
    pub heart_rate: [(f64, f64); 200],
    pub min_y: f64,
    pub max_y: f64,
    pub max_x: f64,
    pub threshold_power: [(f64, f64); 200]
}

impl ChartDataset {
    fn new(power_data: [(f64, f64); 200],
            heart_rate_data: [(f64, f64); 200],
            min_y_value: f64,
            max_y_value: f64,
            max_x_value: f64,
            threshold_power_data: [(f64, f64); 200]) -> Self {
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

pub fn build_session_records_dataset(app: &App) -> ChartDataset {
    let latest_session = app.latest_session.to_owned().unwrap();
   
    let mut power_array: [(f64, f64); 200] = [(0.0, 0.0); 200];
    let mut heart_array: [(f64, f64); 200] = [(0.0, 0.0); 200];
    let mut threshold_power_data: [(f64, f64); 200] = [(0.0, 0.0); 200];

    for (idx, item) in latest_session.records.iter().enumerate() {
        power_array[idx] = (60.0 * idx as f64, item.power as f64);  
        heart_array[idx] = (60.0 * idx as f64, item.heart_rate as f64);
        threshold_power_data[idx] = (60.0 * idx as f64, latest_session.threshold_power as f64);
    }

    let min_value_y = latest_session.records.iter()
        .map(|x| x.heart_rate)
        .collect::<Vec<i64>>()
        .iter()
        .min()
        .unwrap()
        .to_owned() as f64;

    let max_value_y = latest_session.records.iter()
        .map(|x| x.power)
        .collect::<Vec<i64>>()
        .iter()
        .max()
        .unwrap()
        .to_owned() as f64;

    let max_value_x = (latest_session.records.last().unwrap().timestamp - latest_session.records.first().unwrap().timestamp) as f64;

    let dataset = ChartDataset::new(power_array, heart_array, min_value_y, max_value_y, max_value_x, threshold_power_data);
    return dataset;
}
