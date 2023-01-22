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
    pub selected_session: Option<Session>,
    pub selected_session_index: Option<usize>
}

pub struct ChartDataset {
    pub power: [(f64, f64); 9999],
    pub heart_rate: [(f64, f64); 9999],
    pub min_y: f64,
    pub max_y: f64,
    pub max_x: f64,
    pub threshold_power: [(f64, f64); 9999]
}

impl ChartDataset {
    pub fn new(power_data: [(f64, f64); 9999],
            heart_rate_data: [(f64, f64); 9999],
            min_y_value: f64,
            max_y_value: f64,
            max_x_value: f64,
            threshold_power_data: [(f64, f64); 9999]) -> Self {
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
            selected_session_index: Some(0),
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

    pub fn get_summary_by_session_type(&self, session_type: SessionType) {
        todo!();
    }

    pub fn get_current_year_summary(&self) {
        todo!();
    }
}
