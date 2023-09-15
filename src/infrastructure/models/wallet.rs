use crate::domain::models::wallet::Wallet;
use crate::infrastructure::schema::wallets;
use diesel;
use diesel::prelude::*;

#[derive(Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = wallets)]
pub struct WalletDiesel {
    pub address: String,
    pub token_address: String,
    pub balance: i64,
    pub block_number: i32,
    pub transaction_hash: String,
}

impl From<Wallet> for WalletDiesel {
    fn from(t: Wallet) -> Self {
        WalletDiesel {
            address: t.address,
            token_address: t.token_address,
            balance: t.balance,
            block_number: t.block_number,
            transaction_hash: t.transaction_hash,
        }
    }
}

impl Into<Wallet> for WalletDiesel {
    fn into(self) -> Wallet {
        Wallet {
            address: self.address,
            token_address: self.token_address,
            balance: self.balance,
            block_number: self.block_number,
            transaction_hash: self.transaction_hash,
        }
    }
}
