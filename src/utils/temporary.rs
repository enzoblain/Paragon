// This should only be used in development and testing environments.
// It will store the functions that won't be used in production.
use crate::Candle;

use chrono::{DateTime, Utc};
use polars::{prelude::*,
            frame::row::Row
};
use std::fs::File;

pub fn get_data() -> Result<DataFrame, PolarsError> {
    ParquetReader::new(File::open("data/EURUSD.parquet")?)
        .finish()
}

// Because a Row.0 is vec of AnyValue, I iter over it
// The form I have in data is: datetime, open, high, low, close, volume
// It checks the type of each value and returns a Candle struct
pub fn parse_candle(row: Row) -> Result<Candle, String> {
    let datetime = if let AnyValue::Datetime(ts, TimeUnit::Microseconds, _) = row.0[0] {
        DateTime::<Utc>::from_naive_utc_and_offset(
            DateTime::from_timestamp_micros(ts).expect("timestamp invalide").naive_local(),
            Utc,
        )
    } else {
        Err("Invalid 'datetime' value in row: {row}")?
    };

    let open = if let AnyValue::Float64(val) = row.0[1] {
        val
    } else {
        Err("Invalid 'open' value in row: {row}")?
    };

    let high = if let AnyValue::Float64(val) = row.0[2] {
        val
    } else {
        Err("Invalid 'high' value in row: {row}")?
    };

    let low = if let AnyValue::Float64(val) = row.0[3] {
        val
    } else {
        Err("Invalid 'low' value in row: {row}")?
    };

    let close = if let AnyValue::Float64(val) = row.0[4] {
        val
    } else {
        Err("Invalid 'close' value in row: {row}")?
    };

    let volume = if let AnyValue::Int64(val) = row.0[5] {
        val
    } else {
        Err("Invalid 'volume' value in row: {row}")?
    };

    Ok(Candle {
        symbol: "EURUSD".to_string(), // EUR/USD is the only symbol in the data
        timerange: "1m".to_string(),  // 1 minute is the only timerange in the data
        timestamp: datetime,
        open,
        high,
        low,
        close,
        volume: volume as f64,
    })
}