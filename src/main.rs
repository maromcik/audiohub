use crate::database::common::{*};
use crate::database::repositories::{*};
use actix_web::web;
use std::env;
use crate::database::common::{setup_pool, DbPoolHandler, DbRepository};
use actix_web::{App, HttpServer};
use std::sync::Arc;
use env_logger::Env;
use log::{info, warn};
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::user::repository::UserRepository;

mod database;

const DEFAULT_HOSTNAME: &str = "localhost";
const DEFAULT_PORT: &str = "8000";

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Create connection pool
    let pool = Arc::new(setup_pool(10_u32).await?);
    let host = parse_host();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    if let Err(e) = dotenvy::dotenv() {
        warn!("failed loading .env file: {e}");
    };
    info!("starting server on {host}");
    let user_repository = UserRepository::new(PoolHandler::new(pool.clone()));
    let audiobook_repository = AudiobookRepository::new(PoolHandler::new(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_repository.clone()))
    })
        .bind(host)?
        .run()
        .await?;
    Ok(())
}

fn parse_host() -> String {
    let hostname = env::var("HOSTNAME").unwrap_or(DEFAULT_HOSTNAME.to_string());
    let port = env::var("PORT").unwrap_or(DEFAULT_PORT.to_string());
    format!("{hostname}:{port}")
}