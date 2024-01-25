use crate::authorized;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::templates::index::{IndexContentTemplate, IndexTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse, HttpRequest};
use askama::Template;
use crate::database::common::DbReadMany;
use crate::handlers::helpers::get_index_base;

#[get("/")]
pub async fn index(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let base = get_index_base(u, user_repo, book_repo).await?;
    let body = IndexTemplate::from(base).render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/home-content")]
pub async fn index_content(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let base = get_index_base(u, user_repo, book_repo).await?;
    let body = IndexContentTemplate::from(base).render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

