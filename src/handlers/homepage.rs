use crate::authorized;
use crate::database::common::{DbReadMany, DbReadOne};
use crate::database::models::audiobook::{ActiveAudiobookDetail, AudiobookSearch};
use crate::database::models::user::UserGetById;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::handlers::utilities::{get_active_audiobooks, parse_user_id};
use crate::templates::index::{IndexBase, IndexContentTemplate, IndexTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse};
use askama::Template;
use crate::handlers::CONSIDER_AUDIOBOOK_FINISHED;

#[get("/")]
pub async fn index(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let base = get_index_base_template(u, user_repo, book_repo).await?;
    let body = IndexTemplate::from(base).render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/home-content")]
pub async fn index_content(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);

    let base = get_index_base_template(u, user_repo, book_repo).await?;
    let body = IndexContentTemplate::from(base).render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

async fn get_index_base_template(u: Identity, user_repo: web::Data<UserRepository>, book_repo: web::Data<AudiobookRepository>) -> Result<IndexBase, AppError> {
    let mut audiobooks = book_repo.read_many(&AudiobookSearch::default()).await?;
    let user = user_repo
        .read_one(&UserGetById::new(&parse_user_id(u)?))
        .await?;

    let active_audiobooks = get_active_audiobooks(&audiobooks);
    audiobooks.retain(|a|
        a.playback_position.is_some_and(|pos| (a.length - pos) <= CONSIDER_AUDIOBOOK_FINISHED)
            || a.playback_position.is_none());

    let template = IndexBase {
        username: user.name,
        logged_in: true,
        audiobooks,
        active_audiobooks,
    };
    Ok(template)
}