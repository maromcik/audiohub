use crate::authorized;
use crate::error::AppError;
use crate::templates::library::{LibraryContentTemplate, LibraryPageTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse};
use askama::Template;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::handlers::helpers::get_library;

#[get("/library")]
pub async fn index(
    identity: Option<Identity>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let template = LibraryPageTemplate { audiobooks: get_library(u, book_repo).await? };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/library-content")]
pub async fn get_content(
    identity: Option<Identity>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let template = LibraryContentTemplate { audiobooks: get_library(u, book_repo).await? };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
