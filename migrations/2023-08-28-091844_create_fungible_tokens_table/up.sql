CREATE TABLE fungible_tokens (
    address VARCHAR(42) PRIMARY KEY,
    symbol VARCHAR(10) NOT NULL,
    name VARCHAR(66) NOT NULL,
    owner_address VARCHAR(42) NOT NULL,
    decimals INT DEFAULT 0 NOT NULL,
    total_supply BIGINT DEFAULT 0 NOT NULL,
    block_number INTEGER NOT NULL,
    transaction_hash VARCHAR(66) NOT NULL
);
