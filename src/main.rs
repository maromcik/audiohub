use std::sync::Arc;
use crate::database::common::{DbPoolHandler, DbRepository, setup_pool};

mod database;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create connection pool
    let pool = Arc::new(setup_pool(10_u32).await?);
    sqlx::migrate!("./migrations").run(&*pool).await?;
    Ok(())
}
