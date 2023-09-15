use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct FungibleToken {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub owner_address: String,
    pub decimals: i32,
    pub total_supply: i64,
    pub block_number: i32,
    pub transaction_hash: String,
}

#[derive(Clone)]
pub struct UpdatedFungibleToken {
    pub total_supply: Option<i64>,
}
