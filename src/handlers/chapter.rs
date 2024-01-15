use crate::authorized;
use crate::database::common::{DbCreate, DbReadMany};
use crate::database::models::chapter::{ChapterCreate, ChaptersGetByBookId};
use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::error::AppError;
use crate::templates::chapter::{ChapterCreateFormTemplate, ChaptersByAudiobookTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse, post};
use askama::Template;
use sqlx::postgres::types::PgInterval;
use crate::database::models::Id;
use crate::forms::chapter::ChapterCreateForm;
use crate::templates::audiobook::AudiobookDetailCreatorTemplate;

#[get("/create/audiobook/{id}")]
pub async fn create_chapter_form(
    identity: Option<Identity>,
    path: web::Path<(Id,)>
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let template = ChapterCreateFormTemplate { audiobook_id: path.into_inner().0 };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/create")]
pub async fn create_chapter(
    identity: Option<Identity>,
    chapter_repo: web::Data<ChapterRepository>,
    form: web::Form<ChapterCreateForm>)
    -> Result<HttpResponse, AppError> {
    authorized!(identity);
    chapter_repo.create(&ChapterCreate::new(&form.name,
                                            &form.audiobook_id,
                                            &PgInterval {
                                                months: 0,
                                                days: 0,
                                                microseconds: 0
                                            },
                                            &form.sequential_number)).await?;
    Ok(HttpResponse::Ok().finish())
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
