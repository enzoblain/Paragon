use chrono::{DateTime, Utc};

pub struct TwoDStructures {
    pub structure: String,
    pub timerange: String,
    pub timestamp: DateTime<Utc>,
    pub high: f64,
    pub low: f64,
    pub direction: String,
}