use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct Candle {
    pub symbol: String,
    pub timerange: String,
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}