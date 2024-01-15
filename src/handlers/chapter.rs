use crate::authorized;
use crate::database::common::DbReadMany;
use crate::database::models::chapter::ChapterGetByBookId;
use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::error::AppError;
use crate::templates::chapter::{ChapterCreateFormTemplate, ChaptersByAudiobookTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse};
use askama::Template;

#[get("/create")]
pub async fn create_chapter_form(identity: Option<Identity>) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let template = ChapterCreateFormTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/")]
pub async fn get_chapters_by_book(
    identity: Option<Identity>,
    chapter_repo: web::Data<ChapterRepository>,
    params: web::Query<ChapterGetByBookId>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let chapters = chapter_repo
        .read_many(&ChapterGetByBookId {
            audiobook_id: params.audiobook_id,
        })
        .await?;
    let template = ChaptersByAudiobookTemplate { chapters };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
