use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::fungible_token::FungibleToken;
use crate::domain::repositories::repository::ResultPaging;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FungibleTokenDTO {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub owner_address: String,
    pub decimals: i32,
    pub total_supply: i64,
    pub block_number: i32,
    pub transaction_hash: String,
}

impl Into<FungibleTokenDTO> for FungibleToken {
    fn into(self) -> FungibleTokenDTO {
        FungibleTokenDTO {
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

impl Into<ResultPaging<FungibleTokenDTO>> for ResultPaging<FungibleToken> {
    fn into(self) -> ResultPaging<FungibleTokenDTO> {
        ResultPaging {
            total: self.total,
            items: self.items.into_iter().map(|ft| ft.into()).collect(),
        }
    }
}
