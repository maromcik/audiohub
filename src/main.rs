use crate::database::common::*;
use crate::database::common::{setup_pool, DbPoolHandler, DbRepository};
use crate::init::configure_webapp;
use actix_web::{cookie::Key, App, HttpServer};
use env_logger::Env;
use log::{info, warn};
use std::env;
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::http::header;
use actix_web::middleware::Logger;

mod database;
mod error;
mod handlers;
mod init;
mod templates;

const DEFAULT_HOSTNAME: &str = "localhost";
const DEFAULT_PORT: &str = "8000";

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Create connection pool
    let pool = setup_pool(10_u32).await?;
    let host = parse_host();
    let host2 = host.clone();
    let key = Key::generate();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    if let Err(e) = dotenvy::dotenv() {
        warn!("failed loading .env file: {e}");
    };
    info!("starting server on {host}");
    HttpServer::new(move || App::new()
        .wrap(IdentityMiddleware::default())
        .wrap(SessionMiddleware::new(CookieSessionStore::default(), key.clone()))
        .wrap(
            Cors::default()
                .allowed_origin(format!("http://{}", host).as_str())
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .supports_credentials()
                .max_age(3600),
        )
        .wrap(Logger::default())
        .configure(configure_webapp(&pool)))
        .bind(host2)?
        .run()
        .await?;
    Ok(())
}

fn parse_host() -> String {
    let hostname = env::var("HOSTNAME").unwrap_or(DEFAULT_HOSTNAME.to_string());
    let port = env::var("PORT").unwrap_or(DEFAULT_PORT.to_string());
    format!("{hostname}:{port}")
}
