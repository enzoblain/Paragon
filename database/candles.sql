-- This SQL script creates a table for storing candlestick data
-- The table is designed to handle time-series data efficiently.

-- We should use PostgreSQL with TimescaleDB extension for time-series capabilities.
-- CREATE EXTENSION IF NOT EXISTS timescaledb; <-- TODO

CREATE TABLE candles (
    id SERIAL,                             -- Unique identifier for each candle (not primary because of time-series nature)
    symbol TEXT NOT NULL,                  -- Trading pair symbol (e.g., EURUSD)
    timerange TEXT NOT NULL,               -- Timerange for the candle (e.g., 1m, 5m, 1h)
    timestamp TIMESTAMPTZ NOT NULL,        -- Start time of the candle
    open DOUBLE PRECISION NOT NULL,        -- Opening price of the candle
    high DOUBLE PRECISION NOT NULL,        -- Highest price during the candle
    close DOUBLE PRECISION NOT NULL,       -- Closing price of the candle
    low DOUBLE PRECISION NOT NULL,         -- Lowest price during the candle
    volume DOUBLE PRECISION NOT NULL,      -- Trading volume during the candle
    UNIQUE(symbol, timerange, timestamp)   -- Ensure unique entries for each symbol, timerange, and timestamp
);

-- Create a hypertable for time-series data
-- SELECT create_hypertable('candles', 'timestamp'); <-- TODO

-- Create indexes to optimize queries
CREATE INDEX ON candles (symbol, timerange, timestamp DESC);

-- I don't use TimeScaleDB, because I have troubles installing it on my local machine.