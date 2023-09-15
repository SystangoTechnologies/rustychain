use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::domain::models::wallet::Wallet;
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::wallet::{WalletQueryParams, WalletRepository};
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::wallet::WalletDiesel;

pub struct WalletDieselRepository {
    pub pool: Arc<DBConn>,
}

impl WalletDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        WalletDieselRepository { pool: db }
    }
}

#[async_trait]
impl WalletRepository for WalletDieselRepository {
    async fn create_or_update(&self, updated_wallet: &Wallet) -> RepositoryResult<Wallet> {
        use crate::infrastructure::schema::wallets::dsl::{address, token_address, wallets};
        let updated_wallet_diesel: WalletDiesel = WalletDiesel::from(updated_wallet.clone());
        let mut conn = self.pool.get().unwrap();

        let result: WalletDiesel = run(move || {
            diesel::insert_into(wallets)
                .values(updated_wallet_diesel.clone())
                .on_conflict((address, token_address)) // Specify the column for conflict detection
                .do_update() // Perform update if conflict is detected
                .set(updated_wallet_diesel.clone()) // Set the values to update with
                .get_result(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result.into())
    }

    async fn list(&self, params: WalletQueryParams) -> RepositoryResult<ResultPaging<Wallet>> {
        use crate::infrastructure::schema::wallets::dsl::{address, wallets};
        let pool = self.pool.clone();
        let builder = wallets.limit(params.limit()).offset(params.offset()).order_by(address.desc()); // Add order_by clause
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<WalletDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(ResultPaging {
            total: result.len() as i64,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, requested_address: &str, requested_token_address: &str) -> RepositoryResult<Wallet> {
        use crate::infrastructure::schema::wallets::dsl::{address, token_address, wallets};
        let mut conn = self.pool.get().unwrap();
        let requested_address = Arc::new(requested_address.to_string()); // Clone the requested_address using Arc
        let requested_token_address = Arc::new(requested_token_address.to_string());

        run(move || {
            wallets
                .filter(address.eq(requested_address.as_ref()))
                .filter(token_address.eq(requested_token_address.as_ref()))
                .first::<WalletDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())
        .map(|v| -> Wallet { v.into() })
    }
}
