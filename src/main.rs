use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use database::repositories::user::repository::UserRepository;
use database::common::PoolHandler;
mod database;

async fn setup_pool() -> anyhow::Result<Pool<Postgres>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create connection pool
    let pool = Arc::new(setup_pool().await?);
    let _row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool.as_ref())
        .await?;
    // sqlx::migrate!("./migrations").run(&*pool).await?;
    Ok(())
}
