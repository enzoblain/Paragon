use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct TwoDStructures {
    pub symbol: &'static str,
    pub structure: &'static str,
    pub timerange: &'static str,
    pub timestamp: DateTime<Utc>,
    pub high: f64,
    pub low: f64,
    pub direction: &'static str,
}