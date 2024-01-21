use crate::authorized;
use crate::database::common::{DbCreate, DbReadOne};
use crate::database::models::chapter::{ChapterCreate, ChapterGetById};

use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::error::AppError;
use crate::forms::chapter::ChapterCreateForm;
use crate::templates::chapter::{ChapterCreatorPlayerTemplate, ChapterDetailTemplate, ChapterListTemplate, ChapterTimelineTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{post, get, web, HttpResponse, App};
use askama::Template;
use crate::database::models::audiobook::{Audiobook, AudiobookGetById};
use crate::database::models::Id;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::handlers::helpers::transform_to_displayable_chapters;


#[post("/create")]
pub async fn create_chapter(
    identity: Option<Identity>,
    chapter_repo: web::Data<ChapterRepository>,
    form: web::Form<ChapterCreateForm>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    // TODO: check if user is books author!
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

#[get("/audiobook/{book_id}/creator-player")]
pub async fn audio_selection_for_chapter(
    identity: Option<Identity>,
    path: web::Path<Id>,
    audiobook_repo: web::Data<AudiobookRepository>
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let book = audiobook_repo.read_one(&AudiobookGetById{id: path.into_inner()})
        .await?;

    let template = ChapterCreatorPlayerTemplate {source: book.file_path};
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}

#[get("/book/{book_id}/chapter-timeline")]
pub async fn get_chapter_timeline(
    identity: Option<Identity>,
    chapter_repo: web::Data<ChapterRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    path: web::Path<Id>) -> Result<HttpResponse, AppError> {
    authorized!(identity);

    let book_id = path.into_inner();
    let chapters = chapter_repo.get_book_chapters(&AudiobookGetById {id: book_id.clone()}).await?;
    let book = audiobook_repo.read_one(&AudiobookGetById{id: book_id.clone()}).await?;
    let displayable_chapters = transform_to_displayable_chapters(chapters);
    let template = ChapterTimelineTemplate {book_id, chapters: displayable_chapters, length: book.length};
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}


#[get("/book/{book_id}")]
pub async fn get_chapter_list(
    identity: Option<Identity>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<Id>) -> Result<HttpResponse, AppError> {
    authorized!(identity);

    let book_id = path.into_inner();
    let chapters = chapter_repo.get_book_chapters(&AudiobookGetById {id: book_id}).await?;
    let displayable_chapters = transform_to_displayable_chapters(chapters);

    let template = ChapterListTemplate {book_id, chapters: displayable_chapters};
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}

