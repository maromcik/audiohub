use std::env;
use std::sync::Arc;

// use database::common::PoolHandler;
// use database::repositories::user::repository::UserRepository;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool};
// use crate::database::common::{DbPoolHandler, DbRepository};

mod database;

async fn setup_pool() -> anyhow::Result<PgPool> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create connection pool
    let pool = Arc::new(setup_pool().await?);

    // sqlx::migrate!("./migrations").run(&*pool).await?;
    // let mut user_repository = UserRepository::new(PoolHandler::new(pool));

    Ok(())
}
