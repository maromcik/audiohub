use actix_identity::Identity;
use actix_web::{get, HttpResponse, web};
use crate::authorized;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::handlers::utilities::parse_user_id;
use crate::templates::library::{LibraryContentTemplate, LibraryPageTemplate};
use askama::Template;
use actix_web::http::header::LOCATION;

#[get("/library")]
pub async fn index(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let audiobooks = user_repo.get_bookmarked(&parse_user_id(u)?).await?;


    // TODO : change audiobook to active book
    let tmp_book = audiobooks[0].clone();
    let template = LibraryPageTemplate {audiobooks, audiobook:tmp_book};

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/library-content")]
pub async fn get_content(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let audiobooks = user_repo.get_bookmarked(&parse_user_id(u)?).await?;


    // TODO : change audiobook to active book
    let tmp_book = audiobooks[0].clone();
    let template = LibraryContentTemplate {audiobooks, audiobook:tmp_book};

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}