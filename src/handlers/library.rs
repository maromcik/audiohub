use crate::authorized;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::handlers::utilities::parse_user_id;
use crate::templates::library::{LibraryContentTemplate, LibraryPageTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse};
use askama::Template;
use crate::database::models::audiobook::AudiobookDisplay;

#[get("/library")]
pub async fn index(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let audiobooks = user_repo
        .get_bookmarked(&parse_user_id(u)?)
        .await?
        .into_iter()
        .map(AudiobookDisplay::from)
        .collect();

    let template = LibraryPageTemplate { audiobooks };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/library-content")]
pub async fn get_content(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let audiobooks = user_repo
        .get_bookmarked(&parse_user_id(u)?)
        .await?
        .into_iter()
        .map(AudiobookDisplay::from)
        .collect();

    let template = LibraryContentTemplate { audiobooks };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
