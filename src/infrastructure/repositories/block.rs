use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::domain::models::block::{Block, CreateBlock};
use crate::domain::repositories::block::{BlockQueryParams, BlockRepository};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::block::{BlockDiesel, CreateBlockDiesel};

pub struct BlockDieselRepository {
    pub pool: Arc<DBConn>,
}

impl BlockDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        BlockDieselRepository { pool: db }
    }
}

#[async_trait]
impl BlockRepository for BlockDieselRepository {
    async fn create(&self, new_block: &CreateBlock) -> RepositoryResult<Block> {
        use crate::infrastructure::schema::blocks::dsl::blocks;
        let new_block_diesel: CreateBlockDiesel = CreateBlockDiesel::from(new_block.clone());
        let mut conn = self.pool.get().unwrap();
        let result: BlockDiesel = run(move || diesel::insert_into(blocks).values(new_block_diesel).get_result(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.into())
    }

    async fn list(&self, params: BlockQueryParams) -> RepositoryResult<ResultPaging<Block>> {
        use crate::infrastructure::schema::blocks::dsl::{blocks, timestamp};
        let pool = self.pool.clone();
        let builder = blocks.limit(params.limit()).offset(params.offset()).order_by(timestamp.desc()); // Add order_by clause
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<BlockDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(ResultPaging {
            total: result.len() as i64,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, block_nmb: i32) -> RepositoryResult<Block> {
        use crate::infrastructure::schema::blocks::dsl::{block_number, blocks};
        let mut conn = self.pool.get().unwrap();
        run(move || blocks.filter(block_number.eq(block_nmb)).first::<BlockDiesel>(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| -> Block { v.into() })
    }

    async fn delete(&self, block_nmb: i32) -> RepositoryResult<()> {
        use crate::infrastructure::schema::blocks::dsl::{block_number, blocks};
        let mut conn = self.pool.get().unwrap();
        run(move || diesel::delete(blocks).filter(block_number.eq(block_nmb)).execute(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(())
    }
}
