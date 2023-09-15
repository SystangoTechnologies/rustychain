use std::env;

use diesel;
use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;

use crate::domain::constants::{POSTGRESQL_DB_POOL_SIZE_PER_WORKER, POSTGRESQL_DB_URI};

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<diesel::pg::PgConnection>;
pub type DBConn = PostgresPool;

pub fn db_pool() -> DBConn {
    dotenv().ok();
    let database_url = env::var(POSTGRESQL_DB_URI).expect(&*format!("{value} must be set", value = POSTGRESQL_DB_URI));
    let pool_size_per_worker: u32 = env::var(POSTGRESQL_DB_POOL_SIZE_PER_WORKER)
        .expect(&*format!("{value} must be set", value = POSTGRESQL_DB_POOL_SIZE_PER_WORKER))
        .parse()
        .expect("Failed to parse pool size per worker as u32");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().max_size(pool_size_per_worker).build(manager).expect("Failed to create pool")
}
