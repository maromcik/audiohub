use crate::authorized;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::handlers::utilities::parse_user_id;

use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse};
use askama::Template;

use crate::database::models::audiobook::{AudiobookDisplay, AudiobookSearch};
use crate::database::repositories::audiobook::repository::AudiobookRepository;

use crate::database::common::repository::DbReadMany;
use crate::templates::studio::{StudioContentTemplate, StudioPageTemplate};

#[get("/studio")]
pub async fn studio_index(
    identity: Option<Identity>,
    _user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let user_id = parse_user_id(u)?;
    let audiobooks = book_repo
        .read_many(&AudiobookSearch::search_by_author_id(user_id, user_id))
        .await?
        .into_iter()
        .map(AudiobookDisplay::from)
        .collect();

    let template = StudioPageTemplate { audiobooks };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/studio-content")]
pub async fn studio_get_content(
    identity: Option<Identity>,
    _user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let user_id = parse_user_id(u)?;
    let audiobooks = book_repo
        .read_many(&AudiobookSearch::search_by_author_id(user_id, user_id))
        .await?
        .into_iter()
        .map(AudiobookDisplay::from)
        .collect();

    let template = StudioContentTemplate { audiobooks };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
