CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    block_number INTEGER,
    transaction_hash VARCHAR(66) NOT NULL,
    from_address VARCHAR(42) NOT NULL,
    to_address VARCHAR(42) NOT NULL,
    transaction_type VARCHAR(42) NOT NULL,
    value BIGINT NOT NULL,
    timestamp TIMESTAMP,
    data json,
    is_mined BOOLEAN DEFAULT FALSE,
    status VARCHAR(42) NOT NULL,
    CONSTRAINT unique_transaction_hash UNIQUE (transaction_hash)
);
