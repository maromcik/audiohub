use crate::authorized;
use crate::database::common::{DbReadMany, DbReadOne};
use crate::database::models::audiobook::AudiobookSearch;
use crate::database::models::user::UserGetById;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::handlers::utilities::parse_user_id;
use crate::templates::index::{IndexContentTemplate, IndexTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse};
use askama::Template;

#[get("/")]
pub async fn index(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);

    let books = book_repo.read_many(&AudiobookSearch::default()).await?;
    let user = user_repo
        .read_one(&UserGetById::new(&parse_user_id(u)?))
        .await?;

    let template = IndexTemplate {
        username: user.name,
        logged_in: true,
        audiobooks: books,
    };
    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/home-content")]
pub async fn index_content(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);

    let books = book_repo.read_many(&AudiobookSearch::default()).await?;
    let user = user_repo
        .read_one(&UserGetById::new(&parse_user_id(u)?))
        .await?;

    let template = IndexContentTemplate {
        username: user.name,
        logged_in: true,
        audiobooks: books,
    };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
