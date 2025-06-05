-- SQL script to define the 'sessions' table, matching the Rust `Session` struct
-- Suitable for storing session-level trading data (e.g., Asian, London, New York)

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,                 -- Unique identifier
    symbol TEXT NOT NULL,                  -- Trading symbol (e.g., EURUSD)
    label TEXT NOT NULL,                   -- Session label (e.g., Asian, London)
    start_time TIMESTAMPTZ NOT NULL,            -- Start time of the session
    end_time TIMESTAMPTZ NOT NULL,              -- End time of the session
    high DOUBLE PRECISION NOT NULL,        -- Highest price during the session
    low DOUBLE PRECISION NOT NULL,         -- Lowest price during the session
    open DOUBLE PRECISION NOT NULL,        -- Opening price of the session
    close DOUBLE PRECISION NOT NULL,       -- Closing price of the session
    volume DOUBLE PRECISION NOT NULL,      -- Trading volume during the session
    UNIQUE(label, start_time)                   -- Ensure no duplicate sessions
);

-- Index for fast lookup by label and time
CREATE INDEX ON sessions (label, start_time DESC);