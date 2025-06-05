-- SQL script to define the 'two_d_structures' table, matching the Rust `TwoDStructures` struct
-- Suitable for storing structural trading data with time ranges and directional context

CREATE TABLE two_d_structures (
    id SERIAL PRIMARY KEY,                   -- Unique auto-increment identifier
    symbol TEXT NOT NULL,                   -- Trading symbol (e.g., EURUSD)
    structure TEXT NOT NULL,                 -- Name/type of the structure (e.g., FVG)
    timerange TEXT NOT NULL,                 -- Time range label
    timestamp TIMESTAMPTZ NOT NULL,          -- Precise UTC timestamp of the structure
    high DOUBLE PRECISION NOT NULL,          -- Highest recorded value within the structure
    low DOUBLE PRECISION NOT NULL,           -- Lowest recorded value within the structure
    direction TEXT NOT NULL,                 -- Associated direction (e.g., bullish, bearish)
    UNIQUE (structure, timerange, timestamp) -- Prevents exact duplicate entries
);

-- Index to speed up queries by structure, timerange, and descending timestamp
CREATE INDEX ON two_d_structures (structure, timerange, timestamp DESC);