use crate::authorized;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse};
use askama::Template;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::handlers::helpers::get_studio;
use crate::templates::studio::{StudioContentTemplate, StudioPageTemplate};

#[get("/studio")]
pub async fn studio_index(
    identity: Option<Identity>,
    _user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let template = StudioPageTemplate { audiobooks: get_studio(u, book_repo).await? };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/studio-content")]
pub async fn studio_get_content(
    identity: Option<Identity>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let template = StudioContentTemplate { audiobooks: get_studio(u, book_repo).await? };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
