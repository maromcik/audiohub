use crate::authorized;
use crate::database::common::{DbCreate, DbDelete, DbReadOne};
use crate::database::models::chapter::{ChapterCreate, ChapterGetById};

use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::error::AppError;
use crate::forms::chapter::{ChapterCreateForm, ChapterDeleteForm};
use crate::templates::chapter::{ChapterCreatorPlayerTemplate, ChapterListTemplate, ChapterTimelineTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{post, get, web, HttpResponse, delete};
use askama::Template;
use crate::database::common::error::{BackendError, BackendErrorKind};
use crate::database::models::audiobook::{AudiobookGetById};
use crate::database::models::Id;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::handlers::helpers::{get_displayable_chapters};
use crate::handlers::utilities::parse_user_id;


#[post("/create")]
pub async fn create_chapter(
    identity: Option<Identity>,
    chapter_repo: web::Data<ChapterRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    form: web::Form<ChapterCreateForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let audiobook = audiobook_repo.read_one(&AudiobookGetById::new(&form.audiobook_id)).await?;
    if parse_user_id(u)? != audiobook.author_id {
        return Err(AppError::from(BackendError::new(BackendErrorKind::UnauthorizedOperation)));
    }
    chapter_repo
        .create(&ChapterCreate::new(
            &form.name,
            &form.audiobook_id,
            &form.position,
        ))
        .await?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/audiobook/{id}/creator-player")]
pub async fn audio_selection_for_chapter(
    identity: Option<Identity>,
    path: web::Path<Id>,
    audiobook_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let book = audiobook_repo.read_one(&AudiobookGetById { id: path.into_inner() })
        .await?;

    let template = ChapterCreatorPlayerTemplate { source: book.file_path };
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}

#[get("/audiobook/{id}/chapter-timeline")]
pub async fn get_chapter_timeline(
    identity: Option<Identity>,
    chapter_repo: web::Data<ChapterRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    path: web::Path<Id>) -> Result<HttpResponse, AppError> {
    authorized!(identity);

    let audiobook_id = path.into_inner();
    let book = audiobook_repo.read_one(&AudiobookGetById { id: audiobook_id }).await?;
    let displayable_chapters = get_displayable_chapters(chapter_repo, audiobook_id).await?;
    let template = ChapterTimelineTemplate { audiobook_id, chapters: displayable_chapters, length: book.length };
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}


#[get("/audiobook/{id}")]
pub async fn get_chapter_list(
    identity: Option<Identity>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<Id>)
    -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let audiobook_id = path.into_inner();
    let template = ChapterListTemplate { audiobook_id, chapters: get_displayable_chapters(chapter_repo, audiobook_id).await?, show_delete: false };
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}

#[get("/audiobook/{id}/manage")]
pub async fn get_manage_chapter_list(
    identity: Option<Identity>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<Id>)
    -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let audiobook_id = path.into_inner();
    let template = ChapterListTemplate { audiobook_id, chapters: get_displayable_chapters(chapter_repo, audiobook_id).await?, show_delete: true };
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}

#[delete("/delete")]
pub async fn remove_chapter(
    identity: Option<Identity>,
    chapter_repo: web::Data<ChapterRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    form: web::Form<ChapterDeleteForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let user_id = parse_user_id(u)?;
    let audiobook = audiobook_repo.read_one(&AudiobookGetById::new(&form.audiobook_id)).await?;
    if user_id != audiobook.author_id {
        return Err(AppError::from(BackendError::new(BackendErrorKind::UnauthorizedOperation)));
    }
    chapter_repo.delete(&ChapterGetById::new(form.chapter_id)).await?;
    let displayable_chapters = get_displayable_chapters(chapter_repo, audiobook.id).await?;
    let template = ChapterListTemplate { audiobook_id: audiobook.id, chapters: displayable_chapters, show_delete: true };
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}