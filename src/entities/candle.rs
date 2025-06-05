use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Candle {
    pub symbol: &'static str,
    pub timerange: &'static str,
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub direction: &'static str,
}

impl Candle {
    pub fn new(symbol: &'static str, timerange: &'static str, timestamp: DateTime<Utc>, open: f64, high: f64, low: f64, close: f64, volume: f64,
    ) -> Self {
        Candle {
            symbol,
            timerange,
            timestamp,
            open,
            high,
            low,
            close,
            volume,
            direction: get_direction(open, close)
        }
    }
}

pub fn get_direction(open: f64, close: f64) -> &'static str {
    if close > open {
        "bullish"
    } else if close < open {
        "bearish"
    } else {
        "doji"
    }
}