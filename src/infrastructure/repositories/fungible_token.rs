use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::domain::models::fungible_token::{FungibleToken, UpdatedFungibleToken};
use crate::domain::repositories::fungible_token::{FungibleTokenQueryParams, FungibleTokenRepository};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::fungible_token::{FungibleTokenDiesel, UpdatedFungibleTokenDiesel};

pub struct FungibleTokenDieselRepository {
    pub pool: Arc<DBConn>,
}

impl FungibleTokenDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        FungibleTokenDieselRepository { pool: db }
    }
}

#[async_trait]
impl FungibleTokenRepository for FungibleTokenDieselRepository {
    async fn create(&self, new_fungible_token: &FungibleToken) -> RepositoryResult<FungibleToken> {
        use crate::infrastructure::schema::fungible_tokens::dsl::fungible_tokens;
        let new_fungible_token_diesel: FungibleTokenDiesel = FungibleTokenDiesel::from(new_fungible_token.clone());
        let mut conn = self.pool.get().unwrap();
        let result: FungibleTokenDiesel = run(move || diesel::insert_into(fungible_tokens).values(new_fungible_token_diesel).get_result(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.into())
    }

    async fn list(&self, params: FungibleTokenQueryParams) -> RepositoryResult<ResultPaging<FungibleToken>> {
        use crate::infrastructure::schema::fungible_tokens::dsl::fungible_tokens;
        let pool = self.pool.clone();
        let builder = fungible_tokens.limit(params.limit()).offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<FungibleTokenDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(ResultPaging {
            total: result.len() as i64,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, token_address: &str) -> RepositoryResult<FungibleToken> {
        use crate::infrastructure::schema::fungible_tokens::dsl::{address, fungible_tokens};
        let mut conn = self.pool.get().unwrap();
        let requested_address = Arc::new(token_address.to_string()); // Clone the requested_address using Arc

        run(move || fungible_tokens.filter(address.eq(requested_address.as_ref())).first::<FungibleTokenDiesel>(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| -> FungibleToken { v.into() })
    }

    async fn update(&self, token_address: &str, updated_token_data: UpdatedFungibleToken) -> RepositoryResult<FungibleToken> {
        use crate::infrastructure::schema::fungible_tokens::dsl::{address, fungible_tokens};
        let update_token_diesel = UpdatedFungibleTokenDiesel::from(updated_token_data);
        let mut conn = self.pool.get().unwrap();
        let requested_address = Arc::new(token_address.to_string()); // Clone the requested_address using Arc

        let updated_token = run(move || {
            diesel::update(fungible_tokens.filter(address.eq(requested_address.as_ref())))
                .set(&update_token_diesel)
                .get_result::<FungibleTokenDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(updated_token.into())
    }
}
