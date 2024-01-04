use crate::database::common::{setup_pool, DbPoolHandler, DbRepository};
use std::sync::Arc;

mod database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create connection pool
    let pool = Arc::new(setup_pool(10_u32).await?);
    sqlx::migrate!("./migrations").run(&*pool).await?;
    Ok(())
}
