CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    block_number INTEGER,
    transaction_hash VARCHAR(66) NOT NULL,
    value INTEGER NOT NULL,
    timestamp TIMESTAMP,
    data TEXT
);
