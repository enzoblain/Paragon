use crate::{
    Candle, 
    connections::database::add_2_d_structures,
    entities::two_d_structures::TwoDStructures, 
    Timerange
};

use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::sync::Arc;

// We need to store the last 3 candles of each symbol and timerange
// So we can find the fair value gaps
pub static LAST_THREE_CANDLES: Lazy<Arc<DashMap<String, Vec<Arc<Candle>>>>> = Lazy::new(|| {
    Arc::new(DashMap::new())
});

pub async fn processfairvaluegap(candle: Arc<Candle>, symbol: &str, timerange: &Timerange) -> Result<(), String> {
    let key = format!("{}-{}", symbol, timerange.label);

    let last_candles = LAST_THREE_CANDLES
        .get_mut(key.as_str());

    if let Some(mut last_candles) = last_candles {
        // If we already have 3 candles, we remove the oldest one
        if last_candles.len() == 3 {
            last_candles.remove(0);
        }

        // Add the new candle to the list
        last_candles.push(candle.clone());

        // No we have to check if all the candles have the same direction
        // Because if they don't, we can't have a fair value gap
        // We initialize the direction with the first candle's direction
        let mut direction: Option<String> = None;
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
                    direction = Some(candle.direction.clone());
                }
            }
        }

        let mut high: Option<f64> = None;
        let mut low: Option<f64> = None;

        if let Some(direction) = direction.clone() {
            if direction == "bullish".to_string() {
                if last_candles[0].high < last_candles[2].low {
                    high = Some(last_candles[0].high);
                    low = Some(last_candles[2].low);
                }
            } else if direction == "bearish".to_string() {
                if last_candles[0].low > last_candles[2].high {
                    high = Some(last_candles[2].high);
                    low = Some(last_candles[0].low);
                }
            }
        }

        if let (Some(high), Some(low)) = (high, low) {
            let fair_value_gap = TwoDStructures {
                structure: "Fair Value Gap".to_string(),
                timerange: timerange.label.to_string(),
                timestamp: candle.timestamp,
                high,
                low,
                direction: direction.unwrap().clone()
            };

            add_2_d_structures(&fair_value_gap).await?;
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