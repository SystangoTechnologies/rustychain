use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::{models::wallet::Wallet, repositories::repository::ResultPaging};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WalletDTO {
    pub wallet_address: String,
    pub token_address: String,
    pub balance: i64,
    pub updated_at_block_number: i32,
    pub updated_by_transaction_hash: String,
}

impl Into<WalletDTO> for Wallet {
    fn into(self) -> WalletDTO {
        WalletDTO {
            wallet_address: self.address,
            token_address: self.token_address,
            balance: self.balance,
            updated_at_block_number: self.block_number,
            updated_by_transaction_hash: self.transaction_hash,
        }
    }
}

impl Into<ResultPaging<WalletDTO>> for ResultPaging<Wallet> {
    fn into(self) -> ResultPaging<WalletDTO> {
        ResultPaging {
            total: self.total,
            items: self.items.into_iter().map(|block| block.into()).collect(),
        }
    }
}
