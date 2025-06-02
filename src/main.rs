use paragon::{
    connections::{
        database::init_pool,
        websocket::create_intra_websocket,
    },
    handlers::candle::aggregate_candle,
    TIMERANGES,
    utils::temporary,
};

use futures::future::join_all;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Initialize the websocket server
    let intra_websocket = tokio::spawn(async move{
        create_intra_websocket().await
            .map_err(|e| format!("WebSocket error: {}", e))
    });

    // Spawn the main tasj
    let main_task = tokio::spawn(async move {   
        print!("Starting Paragon...\n");

        // Create the database connection pool
        init_pool().await
            .map_err(|e| format!("Database connection error: {}", e))?;

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

        Ok::<(), String>(())
    });

    // Run both tasks concurrently and handle their results
    tokio::select! {
        res = intra_websocket => match res {
            Ok(Ok(())) => return Err("WebSocket finished without error but too early".into()),
            Ok(Err(e)) => return Err(format!("WebSocket error: {}", e)),
            Err(e) => return Err(format!("WebSocket panic : {}", e)),
        },
        res = main_task => match res {
            Ok(Ok(())) => return Err("Main task finished without error but too early".into()),
            Ok(Err(e)) => return Err(format!("Main task error: {}", e)),
            Err(e) => return Err(format!("Main panic : {}", e)),
        },
    }
}
