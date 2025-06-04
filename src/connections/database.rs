use crate::{Candle, Session};

use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use once_cell::sync::OnceCell;
use tokio_postgres::NoTls;

pub static POOL: OnceCell<Pool> = OnceCell::new();

// Initialize the database connection pool
pub async fn init_pool() -> Result<(), String> {
    // Configure the database connection
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.user = Some("enzoblain".to_string());
    cfg.dbname = Some("Paragon".to_string());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    // Create the pool
    let pool = cfg.create_pool(None, NoTls).map_err(|e| format!("Failed to create database pool: {}", e))?;
    POOL.set(pool).map_err(|_| "Pool already initialized")?;
    Ok(())
}

// Facilitate access to the database client
pub async fn get_db_client() -> Result<deadpool_postgres::Client, String> {
    let pool = POOL.get().ok_or("Pool not initialized")?;
    let client = pool.get().await.map_err(|e| format!("Failed to get database client: {}", e))?;
    
    Ok(client)
}

pub async fn add_candle(candle: &Candle) -> Result<(), String> {
    let client = get_db_client().await?;

    let query = "INSERT INTO candles (symbol, timerange, timestamp, open, high, low, close, volume, direction) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";
    
    client.query(query, &[
        &candle.symbol,
        &candle.timerange,
        &candle.timestamp,
        &candle.open,
        &candle.high,
        &candle.low,
        &candle.close,
        &candle.volume,
        &candle.direction
    ]).await.map_err(|e| format!("Failed to insert candle into database: {}", e))?;

    Ok(())
}

pub async fn add_session(session: &Session) -> Result<(), String> {
    let client = get_db_client().await?;

    let query = "INSERT INTO sessions (label, start_time, end_time, high, low, open, close, volume) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";
    
    client.query(query, &[
        &session.label,
        &session.start,
        &session.end,
        &session.high,
        &session.low,
        &session.open,
        &session.close,
        &session.volume
    ]).await.map_err(|e| format!("Failed to insert session into database: {}", e))?;

    Ok(())
}