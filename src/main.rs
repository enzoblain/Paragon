use paragon::{
    connections::database::init_pool,
    handlers::candle::aggregate_candle,
    TIMERANGES,
    utils::temporary,
};

use futures::future::join_all;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Create the database connection pool
    init_pool().await?;

    // Load the data
    let data = temporary::get_data().map_err(|e| e.to_string())?;

    // Iterate over each row in the data
    for index in 0..data.height() {
        let row = data.get_row(index).map_err(|e| e.to_string())?;
        let parsed_candle = temporary::parse_candle(row).map_err(|e| e.to_string())?;

        let candle = Arc::new(parsed_candle);

        // Spawn a task for each timerange to aggregate the candle
        let mut handles = Vec::new();
        for timerange in TIMERANGES.iter() {
            let cloned_candle = Arc::clone(&candle);
            let task = tokio::spawn(async move {
                aggregate_candle(cloned_candle, "EURUSD", timerange).await
            });

            handles.push(task);
        }

        // Wait for all tasks to complete
        let _ = join_all(handles).await; 
    }

    Ok(())
}
