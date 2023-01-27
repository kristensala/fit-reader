use anyhow::Result;

use crate::db;

#[derive(Clone)]
pub struct Summary {
    pub sub_sport: Option<String>,
    pub total_distance: f64,
    pub total_time: f64,
    pub rides_count: i64
}

impl Summary {
    pub fn new(sub_sport: String) -> Self {
        return Self {
            sub_sport: Some(sub_sport),
            total_distance: 0.0,
            total_time: 0.0,
            rides_count: 0
        };
    }
    /// Gets summary for each sub sport separately
    pub fn detailed(year: i64) -> Result<Vec<Summary>> {
        let summary_result = db::get_detailed_summary(year);

        if summary_result.is_ok() {
            return summary_result;
        }
        return Ok(Vec::new());
    }

    pub fn overall(year: i64) -> Result<Summary> {
        let summary_result = db::get_overall_summary(year);

        if summary_result.is_ok() {
            return summary_result;
        }

        return Ok(Summary {
            sub_sport: None,
            total_distance: 0.0,
            total_time: 0.0,
            rides_count: 0
        });
    }
}
