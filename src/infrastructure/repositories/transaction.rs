use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::domain::models::transaction::{CreateTransaction, Transaction, UpdateTransaction};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::transaction::{TransactionQueryParams, TransactionRepository};
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::transaction::{CreateTransactionDiesel, TransactionDiesel, UpdateTransactionDiesel};

pub struct TransactionDieselRepository {
    pub pool: Arc<DBConn>,
}

impl TransactionDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        TransactionDieselRepository { pool: db }
    }
}

#[async_trait]
impl TransactionRepository for TransactionDieselRepository {
    async fn create(&self, new_transaction: &CreateTransaction) -> RepositoryResult<Transaction> {
        use crate::infrastructure::schema::transactions::dsl::transactions;
        let new_transaction_diesel: CreateTransactionDiesel = CreateTransactionDiesel::from(new_transaction.clone());
        let mut conn = self.pool.get().unwrap();
        let result: TransactionDiesel = run(move || diesel::insert_into(transactions).values(new_transaction_diesel).get_result(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.into())
    }

    async fn list(&self, params: TransactionQueryParams) -> RepositoryResult<ResultPaging<Transaction>> {
        use crate::infrastructure::schema::transactions::dsl::{is_mined, transactions};
        let pool = self.pool.clone();
        let builder = transactions.limit(params.limit()).offset(params.offset()).filter(is_mined.eq(params.is_mined.unwrap_or(false))); // Default to false

        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<TransactionDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(ResultPaging {
            total: result.len() as i64,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, transaction_hash_val: &str) -> RepositoryResult<Transaction> {
        use crate::infrastructure::schema::transactions::dsl::{transaction_hash, transactions};
        let mut conn = self.pool.get().unwrap();
        let requested_hash = Arc::new(transaction_hash_val.to_string()); // Clone the requested_address using Arc
        run(move || transactions.filter(transaction_hash.eq(requested_hash.as_ref())).first::<TransactionDiesel>(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| -> Transaction { v.into() })
    }

    async fn delete(&self, transaction_id: i32) -> RepositoryResult<()> {
        use crate::infrastructure::schema::transactions::dsl::{id, transactions};
        let mut conn = self.pool.get().unwrap();
        run(move || diesel::delete(transactions).filter(id.eq(transaction_id)).execute(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(())
    }

    async fn update(&self, transaction_id: i32, update_data: UpdateTransaction) -> RepositoryResult<Transaction> {
        use crate::infrastructure::schema::transactions::dsl::{id, transactions};
        let update_transaction_diesel = UpdateTransactionDiesel::from(update_data);
        let mut conn = self.pool.get().unwrap();
        let updated_transaction = run(move || {
            diesel::update(transactions.filter(id.eq(transaction_id)))
                .set(&update_transaction_diesel)
                .get_result::<TransactionDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(updated_transaction.into())
    }
}
