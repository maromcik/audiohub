use crate::database::common::setup_pool;
use crate::init::configure_webapp;
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_multipart::form::MultipartFormConfig;
use actix_session::config::CookieContentSecurity;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::SameSite;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::web::PayloadConfig;
use actix_web::{cookie::Key, App, HttpServer};
use env_logger::Env;
use log::{info, warn};
use std::env;

mod database;
mod error;
mod forms;
mod handlers;
mod init;
mod templates;
mod utilities;

const DEFAULT_HOSTNAME: &str = "localhost";
const DEFAULT_PORT: &str = "8000";

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("TMPDIR", "./media");
    let dir = env::temp_dir();
    println!("Temporary directory: {}", dir.display());

    let pool = setup_pool(10_u32).await?;
    let host = parse_host();
    let host2 = host.clone();

    let key = Key::from(
        &env::var("COOKIE_SESSION_KEY")
            .unwrap_or_default()
            .bytes()
            .collect::<Vec<u8>>(),
    );
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    if let Err(e) = dotenvy::dotenv() {
        warn!("failed loading .env file: {e}");
    };
    info!("starting server on {host}");

    gst::init()?;

    HttpServer::new(move || {
        App::new()
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(16 * 1024 * 1024 * 1024)
                    .memory_limit(16 * 1024 * 1024 * 1024),
            )
            .app_data(PayloadConfig::new(16 * 1024 * 1024 * 1024))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_same_site(SameSite::None)
                    .cookie_http_only(false)
                    .cookie_secure(false)
                    .cookie_content_security(CookieContentSecurity::Private)
                    .build(),
            )
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
            .configure(configure_webapp(&pool))
    })
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
