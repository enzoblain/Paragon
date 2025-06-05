use crate::{
    Candle,
    connections::{
        database::add_candle,
        websocket::send_message_to_clients,
    },
    handlers::structures::processfairvaluegap,
    Timerange,
};

use chrono::{Utc, TimeZone};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde_json::{Map, to_value, Value};
use std::sync::Arc;

// Here we're using DashMap to allow concurrent access to the candles
// Because we are sure that we won't use the same key in multiple threads
// Currently, candles are stored using keys in the format "symbol-timerange".
// However, we should explore alternatives to group timeranges under each symbol
// without sacrificing performance or concurrency.
pub static CANDLES: Lazy<Arc<DashMap<String, Arc<Candle>>>> = Lazy::new(|| {
    Arc::new(DashMap::new())
});

// Aggregates a 1-minute candle into its corresponding higher timeframe candle (5m, 15m, etc.).
pub async fn aggregate_candle(candle: Arc<Candle>, symbol: &'static str, timerange: &Timerange) {
    let key = format!("{}-{}", symbol, timerange.label);

    let last_candle = CANDLES
        .get(key.as_str())
        .map(|c| Arc::clone(c.value()));

    let new_candle;

    // Check if it's the first candle for this timerange
    // If there is no last candle, we create a new one
    // If there is a last candle, we check if the new candle is in the same timerange 
    if let Some(last_candle) = last_candle {
        if last_candle.timestamp + chrono::Duration::milliseconds(timerange.duration_ms as i64) <= candle.timestamp {
            // Send the candle to the db and check for errors
            if let Err(e) = add_candle(&last_candle).await {
                eprintln!("Failed to add candle to database: {}", e);
            }

            // Send the candle to the websocket
            if let Err(e) = send_candle(&last_candle).await {
                eprintln!("Failed to send candle to websocket: {}", e);
            }

            // Search for fair value gaps
            if let Err(e) = processfairvaluegap(Arc::clone(&last_candle), symbol, timerange).await {
                eprintln!("Failed to process fair value gap: {}", e);
            }

            // Update the dashmap with the new candle (change the timerange)
            let mut modified_candle = (*candle).clone();
            modified_candle.timerange = timerange.label;

            // Adjust the open price to match the timerange,
            // So for example the open 
            modified_candle.timestamp = Utc.timestamp_millis_opt((modified_candle.timestamp.timestamp_millis() / timerange.duration_ms as i64) * timerange.duration_ms as i64).single().expect("Failed to adjust timestamp");

            new_candle = Arc::new(modified_candle);
        } else {
            // If the new candle is in the same timerange
            // Take the last candle and update it with the new candle
            let mut modified_candle = (*last_candle).clone();

            // Update the candle with the new values
            modified_candle.high = modified_candle.high.max(candle.high);
            modified_candle.low = modified_candle.low.min(candle.low);
            modified_candle.close = candle.close;
            modified_candle.volume += candle.volume;

            new_candle = Arc::new(modified_candle);
        }
    } else {
        // Don't forget to change the timerange of the candle
        let mut candle = (*candle).clone();
        candle.timerange = timerange.label;
        // And adjust the timestamp to match the timerange
        // This is done by rounding the timestamp to the nearest timerange duration
        candle.timestamp = Utc.timestamp_millis_opt((candle.timestamp.timestamp_millis() / timerange.duration_ms as i64) * timerange.duration_ms as i64).single().expect("Failed to adjust timestamp");

        new_candle = Arc::new(candle);
    }

    // Send the candle to the websocket
    if let Err(e) = send_candle(&new_candle).await {
        eprintln!("Failed to send candle to websocket: {}", e);
    }

    // Insert or update the candle in the DashMap
    CANDLES
        .entry(key)
        .and_modify(|c| *c = Arc::clone(&new_candle))
        .or_insert_with(|| Arc::clone(&new_candle));
}

// Sends a candle to the connected WebSocket clients.
// By converting the candle to a JSON string, we can send it over the WebSocket connection.
pub async fn send_candle(candle: &Candle) -> Result<(), String> {
    let mut data = Map::new();

    // Structure the data to send
    data.insert("type".to_string(), Value::String("candle".to_string()));
    data.insert("value".to_string(), to_value(candle).unwrap());

    // Convert the data to a JSON string
    let json_data = Value::Object(data).to_string();

    // Send the data to the clients
    send_message_to_clients(&json_data).await?;

    Ok(())
}