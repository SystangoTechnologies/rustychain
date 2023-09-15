use crate::domain::models::fungible_token::{FungibleToken, UpdatedFungibleToken};
use crate::infrastructure::schema::fungible_tokens;

use diesel;
use diesel::prelude::*;

#[derive(Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = fungible_tokens)]
pub struct FungibleTokenDiesel {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub owner_address: String,
    pub decimals: i32,
    pub total_supply: i64,
    pub block_number: i32,
    pub transaction_hash: String,
}

impl From<FungibleToken> for FungibleTokenDiesel {
    fn from(t: FungibleToken) -> Self {
        FungibleTokenDiesel {
            address: t.address,
            symbol: t.symbol,
            name: t.name,
            owner_address: t.owner_address,
            decimals: t.decimals,
            total_supply: t.total_supply,
            block_number: t.block_number,
            transaction_hash: t.transaction_hash,
        }
    }
}

impl Into<FungibleToken> for FungibleTokenDiesel {
    fn into(self) -> FungibleToken {
        FungibleToken {
            address: self.address,
            symbol: self.symbol,
            name: self.name,
            owner_address: self.owner_address,
            decimals: self.decimals,
            total_supply: self.total_supply,
            block_number: self.block_number,
            transaction_hash: self.transaction_hash,
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = fungible_tokens)]
pub struct UpdatedFungibleTokenDiesel {
    pub total_supply: Option<i64>,
}

impl From<UpdatedFungibleToken> for UpdatedFungibleTokenDiesel {
    fn from(u: UpdatedFungibleToken) -> Self {
        UpdatedFungibleTokenDiesel { total_supply: u.total_supply }
    }
}
