-- SQL script to define the 'trends' table, matching the Rust `Trend` struct
-- Suitable for storing trend-level data across time ranges

CREATE TABLE trends (
    id SERIAL PRIMARY KEY,                         -- Unique identifier
    symbol TEXT NOT NULL,                          -- Trading symbol (e.g., EURUSD)
    timerange TEXT NOT NULL,                       -- Timeframe (e.g., 1h, 4h, 1d)
    start_time TIMESTAMPTZ NOT NULL,               -- Start time of the trend
    end_time TIMESTAMPTZ NOT NULL,                 -- End time of the trend
    direction TEXT NOT NULL,                       -- Trend direction (e.g., bullish, bearish)
    high DOUBLE PRECISION NOT NULL,                -- Absolute high during the trend
    low DOUBLE PRECISION NOT NULL,                 -- Absolute low during the trend
    UNIQUE(symbol, timerange, start_time)          -- Prevent duplicate trends
);

-- Index for efficient querying by symbol and timeframe
CREATE INDEX ON trends (symbol, timerange, start_time DESC);