use crate::authorized;
use crate::database::common::DbReadMany;
use crate::database::models::chapter::ChaptersGetByBookId;
use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::error::AppError;
use crate::templates::chapter::{ChapterCreateFormTemplate, ChaptersByAudiobookTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse, post};
use askama::Template;
use crate::database::models::Id;

#[get("/create")]
pub async fn create_chapter_form(identity: Option<Identity>) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let template = ChapterCreateFormTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/create")]
pub async fn create_chapter(identity: Option<Identity>, chapter_repo: web::Data<ChapterRepository>) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    todo!()
}

#[get("/audiobook/{id}")]
pub async fn get_chapters_by_audiobook(
    identity: Option<Identity>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let chapters = chapter_repo
        .read_many(&ChaptersGetByBookId {
            audiobook_id: path.into_inner().0,
        })
        .await?;
    let template = ChaptersByAudiobookTemplate { chapters };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
