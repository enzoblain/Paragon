use crate::Candle;

use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::NoTls;

pub static DB_CLIENT: OnceCell<Arc<Mutex<tokio_postgres::Client>>> = OnceCell::new();

pub async fn connect_db() -> Result<(), String> {
    if DB_CLIENT.get().is_some() {
        // If the database client is already initialized, don't need to do anything
        return Ok(());
    }

    // Create a connection to the PostgreSQL database
    let (client, connection) = tokio_postgres::connect("host=localhost user=enzoblain dbname=Paragon", NoTls).await.map_err(|e| format!("Failed to connect to the database: {}", e))?;

    // Spawn a new task to run the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Set the client in the OnceCell
    DB_CLIENT.set(Arc::new(Mutex::new(client))).map_err(|_| "Failed to set the database client".to_string())?;

    Ok(())
}

// Facilitate access to the database client
pub async fn get_db_client() -> Result<Arc<Mutex<tokio_postgres::Client>>, String> {
    DB_CLIENT.get().ok_or_else(|| "Database client not initialized".to_string()).map(|client| client.clone())
}

pub async fn add_candle(candle: &Candle) -> Result<(), String> {
    let client = get_db_client().await?;

    let query = "INSERT INTO candles (symbol, timerange, timestamp, open, high, low, close, volume) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";
    
    client.lock().await.execute(query, &[
        &candle.symbol,
        &candle.timerange,
        &candle.timestamp,
        &candle.open,
        &candle.high,
        &candle.low,
        &candle.close,
        &candle.volume,
    ]).await.map_err(|e| format!("Failed to insert candle into database: {}", e))?;

    Ok(())
}