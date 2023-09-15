CREATE TABLE blocks (
    block_number SERIAL PRIMARY KEY,
    block_hash VARCHAR(66) NOT NULL,
    parent_hash VARCHAR(66) NOT NULL,
    timestamp TIMESTAMP,
    miner_address VARCHAR(42) NOT NULL,
    transaction_count INTEGER NOT NULL
);