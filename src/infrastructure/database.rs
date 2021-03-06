use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::pool::Pool;
use sqlx::postgres::{PgPoolOptions, Postgres};
use std::env;

use crate::error::{Error, Result};

const DB_POOL_MAX_CONNECTIONS: u32 = 5;

pub type DbPool = Pool<Postgres>;

lazy_static! {
    static ref DB_POOL: AsyncOnce<DbPool> = AsyncOnce::new(async {
        create_pool()
            .await
            .expect("Unable to create a database pool")
    });
}

pub async fn create_pool() -> Result<Pool<Postgres>> {
    let db_uri = env::var("DATABASE_URL").expect("Missing \"DATABASE_URL\" environment variable");

    PgPoolOptions::new()
        .max_connections(DB_POOL_MAX_CONNECTIONS)
        .connect(&db_uri)
        .await
        .map_err(Error::from)
}

pub async fn get_db_pool() -> &'static DbPool {
    DB_POOL.get().await
}

pub async fn ping() -> Result<()> {
    let pool = get_db_pool().await;

    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .expect("Failed to PING database");

    Ok(())
}
