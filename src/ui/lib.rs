use crate::app::App;

pub struct ChartDataset {
    pub power: [(f64, f64); 200],
    pub heart_rate: [(f64, f64); 200],
    pub min_y: f64,
    pub max_y: f64
}

impl ChartDataset {
    fn new(power_data: [(f64, f64); 200],
            heart_rate_data: [(f64, f64); 200],
            min_y_value: f64,
            max_y_value: f64) -> Self {
        return Self {
            power: power_data,
            heart_rate: heart_rate_data,
            min_y: min_y_value, 
            max_y: max_y_value
        }
    }
}

pub fn build_session_records_dataset(app: &App) -> ChartDataset {
    let records = app.latest_session.to_owned().unwrap().records;
   
    let mut power_array: [(f64, f64); 200] = [(0.0, 0.0); 200];
    let mut heart_array: [(f64, f64); 200] = [(0.0, 0.0); 200];

    for (idx, item) in records.iter().enumerate() {
        power_array[idx] = (60.0 * idx as f64, item.power as f64);  
        heart_array[idx] = (60.0 * idx as f64, item.heart_rate as f64);  
    }

    let min_value_y = records.iter()
        .map(|x| x.heart_rate)
        .collect::<Vec<i64>>()
        .iter()
        .min()
        .unwrap()
        .to_owned() as f64;

    let max_value_y = records.iter()
        .map(|x| x.power)
        .collect::<Vec<i64>>()
        .iter()
        .max()
        .unwrap()
        .to_owned() as f64;

    let dataset = ChartDataset::new(power_array, heart_array, min_value_y, max_value_y);
    return dataset;
}
