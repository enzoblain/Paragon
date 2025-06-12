use crate::Candle;

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Trend {
    pub symbol: &'static str,
    pub timerange: &'static str,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub direction: &'static str,
    pub high: f64,
    pub low: f64,
    pub high_datetime: DateTime<Utc>,
    pub low_datetime: DateTime<Utc>,
    pub relative_high: f64,
    pub relative_low: f64,
}

#[derive(Clone)]
pub struct Subtrend {
    pub start_time: DateTime<Utc>,
    pub direction: &'static str,
    pub high: f64,
    pub low: f64,
    pub last_relative_low: f64,
    pub last_relative_high: f64,
    pub last_candle: Candle,
    pub last_relative_low_datetime: DateTime<Utc>,
    pub last_relative_high_datetime: DateTime<Utc>,
}