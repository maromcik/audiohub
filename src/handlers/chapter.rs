use crate::authorized;
use crate::database::common::{DbCreate, DbReadMany};
use crate::database::models::chapter::{ChapterCreate, ChaptersGetByBookId};
use crate::database::models::Id;
use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::error::AppError;
use crate::forms::chapter::{ChapterCreateAudiobookInfoForm, ChapterCreateForm};
use crate::templates::chapter::{
    ChapterCreateFormTemplate, ChapterDetailTemplate, ChaptersByAudiobookTemplate,
};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, HttpResponse};
use askama::Template;

#[post("/create/form")]
pub async fn create_chapter_form(
    identity: Option<Identity>,
    form: web::Form<ChapterCreateAudiobookInfoForm>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let template = ChapterCreateFormTemplate {
        audiobook_id: form.audiobook_id,
        position: form.position,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/create")]
pub async fn create_chapter(
    identity: Option<Identity>,
    chapter_repo: web::Data<ChapterRepository>,
    form: web::Form<ChapterCreateForm>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let chapter = chapter_repo
        .create(&ChapterCreate::new(
            &form.name,
            &form.audiobook_id,
            &form.position,
        ))
        .await?;

    let template = ChapterDetailTemplate { chapter };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
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
