use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub token_address: String,
    pub balance: i64,
    pub block_number: i32,
    pub transaction_hash: String,
}
