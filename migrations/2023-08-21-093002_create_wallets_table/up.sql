CREATE TABLE wallets (
    address VARCHAR(42),
    token_address VARCHAR(42),
    balance BIGINT NOT NULL,
    block_number INTEGER NOT NULL,
    transaction_hash VARCHAR(66) NOT NULL,
    PRIMARY KEY (address, token_address)
);
