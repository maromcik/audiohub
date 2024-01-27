use crate::authorized;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::error::AppError;
use crate::handlers::helpers::get_library;
use crate::templates::library::{LibraryContentTemplate, LibraryPageTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;

#[get("/library")]
pub async fn index(
    request: HttpRequest,
    identity: Option<Identity>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let template = LibraryPageTemplate {
        audiobooks: get_library(u, book_repo).await?,
    };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/library-content")]
pub async fn get_content(
    request: HttpRequest,
    identity: Option<Identity>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let template = LibraryContentTemplate {
        audiobooks: get_library(u, book_repo).await?,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
