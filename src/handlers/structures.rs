use crate::{
    connections::{database::add_2_d_structures, websocket::send_message_to_clients}, entities::structures::TwoDStructures, Candle, Timerange
};

use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde_json::{Map, to_value, Value};
use std::sync::Arc;

// We need to store the last 3 candles of each symbol and timerange
// So we can find the fair value gaps
pub static LAST_THREE_CANDLES: Lazy<Arc<DashMap<String, Vec<Arc<Candle>>>>> = Lazy::new(|| {
    Arc::new(DashMap::new())
});

// This function sends a TwoDStructures entity to all connected clients via WebSocket
pub async fn send_two_d_structure(structure: &TwoDStructures) -> Result<(), String> {
    let mut data = Map::new();

    data.insert("type".to_string(), Value::String("Two dimension structure".to_string())); 
    data.insert("value".to_string(), to_value(structure).unwrap());

    let json_data = Value::Object(data).to_string();

    send_message_to_clients(&json_data).await?;

    Ok(())
}

pub async fn processfairvaluegap(candle: Arc<Candle>, symbol: &'static str, timerange: &Timerange) -> Result<(), String> {
    let key = format!("{}-{}", symbol, timerange.label);

    let last_candles = LAST_THREE_CANDLES
        .get_mut(key.as_str());

    if let Some(mut last_candles) = last_candles {
        // If we already have 3 candles, we remove the oldest one
        if last_candles.len() == 3 {
            last_candles.remove(0);
        } else if last_candles.len() > 3 {
            return Err("Too many candles in the list".to_string());
        }

        // Add the new candle to the list
        last_candles.push(candle.clone());

        if last_candles.len() < 3 {
            // If we don't have enough candles, we can't find a fair value gap
            return Ok(());
        }

        // No we have to check if all the candles have the same direction
        // Because if they don't, we can't have a fair value gap
        // We initialize the direction with the first candle's direction
        let mut direction: Option<&'static str> = None;
        for candle in last_candles.iter() {
            // We check if the direction is already initialized
            if let Some(ref direction) = direction {
                // And if the actual candle has the same direction
                if *direction != candle.direction && candle.direction != "doji".to_string() {
                    return Ok(());
                }
            } else {
                // If not we initialize the direction
                // We ignore doji candles for the direction
                if *candle.direction != "doji".to_string() {
                    direction = Some(candle.direction);
                }
            }
        }

        // Store high and low for the fair value gap
        // So we know if we have found one
        let mut high: Option<f64> = None;
        let mut low: Option<f64> = None;

        // If we have a direction, we can check for fair value gaps
        if let Some(direction) = direction.clone() {
            // If it's bullish, we have to find a hole between the first candle shadow and the third candle body
            if direction == "bullish" {
                if last_candles[0].high < last_candles[2].low {
                    high = Some(last_candles[2].low);
                    low = Some(last_candles[0].high);
                }
            // If it's bearish, we have to find a hole between the first candle body and the third candle shadow
            } else if direction == "bearish" {
                if last_candles[0].low > last_candles[2].high {
                    high = Some(last_candles[0].low);
                    low = Some(last_candles[2].high);
                }
            }
        }

        // If we have found a fair value gap, we create a TwoDStructures entity
        // And we add it to the database
        if let (Some(high), Some(low)) = (high, low) {
            let fair_value_gap = TwoDStructures {
                symbol: symbol,
                structure: "Fair Value Gap",
                timerange: timerange.label,
                timestamp: candle.timestamp,
                high,
                low,
                direction: direction.unwrap_or("doji"), // But this should never happen
            };

            add_2_d_structures(&fair_value_gap).await?;

            send_two_d_structure(&fair_value_gap).await?;
        }

        return Ok(());
    } else {
        // If there are no candles yet, we create a new vector
        let mut new_candles = Vec::new();
        new_candles.push(candle);
        LAST_THREE_CANDLES.insert(key, new_candles);
    }

    Ok(())
}