use crate::authorized;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::handlers::utilities::parse_user_id;
use crate::templates::library::{LibraryContentTemplate, LibraryPageTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse};
use askama::Template;
use crate::database::common::query_parameters::DbQueryParams;
use crate::database::models::audiobook::AudiobookSearch;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::templates::studio::{StudioPageTemplate, StudioContentTemplate};
use crate::database::common::repository::DbReadMany;

#[get("/studio")]
pub async fn studio_index(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let user_id = parse_user_id(u)?;
    let audiobooks = book_repo
        .read_many(&AudiobookSearch::search_by_author_id(user_id, user_id))
        .await?;

    let template = StudioPageTemplate { audiobooks };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/studio-content")]
pub async fn studio_get_content(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let user_id = parse_user_id(u)?;
    let audiobooks = book_repo
        .read_many(&AudiobookSearch::search_by_author_id(user_id, user_id))
        .await?;

    let template = StudioContentTemplate { audiobooks };

    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}