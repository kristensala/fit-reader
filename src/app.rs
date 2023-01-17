use crate::parser::Session;

pub enum SessionType {
    Indoor,
    Road,
    MTB
}

pub struct Summary {
    total_distance: f64,
    total_time: String, // HH:MM
    session_type: SessionType
}

pub struct App {
    pub latest_session: Option<Session>,
    pub sessions: Vec<Session>,
    pub selected_session: Option<Session>
}

pub struct ChartDataset {
    pub power: [(f64, f64); 999],
    pub heart_rate: [(f64, f64); 999],
    pub min_y: f64,
    pub max_y: f64,
    pub max_x: f64,
    pub threshold_power: [(f64, f64); 999]
}

impl ChartDataset {
    fn new(power_data: [(f64, f64); 999],
            heart_rate_data: [(f64, f64); 999],
            min_y_value: f64,
            max_y_value: f64,
            max_x_value: f64,
            threshold_power_data: [(f64, f64); 999]) -> Self {
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

impl Default for App {
    fn default() -> Self {
        App {
            latest_session: None,
            sessions: Vec::new(),
            selected_session: None
        }
    }
}

impl App {
    pub fn new() -> Self{
        Self {
            ..App::default()
        }
    }

    pub fn select_session(&mut self, session: Option<Session>) {
        self.selected_session = session;
    }

    pub fn get_summary_by_session_type(&self, session_type: SessionType) {
        todo!();
    }

    pub fn get_current_year_summary(&self) {
        todo!();
    }

    pub fn build_session_dataset(&self) -> ChartDataset {
        let session = self.selected_session.to_owned().unwrap();

        let mut power_array: [(f64, f64); 999] = [(0.0, 0.0); 999];
        let mut heart_array: [(f64, f64); 999] = [(0.0, 0.0); 999];
        let mut threshold_power_data: [(f64, f64); 999] = [(0.0, 0.0); 999];

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
}
