use std::fs::File;
use crate::database::common::{DbCreate, DbReadMany, DbReadOne, DbUpdate};
use crate::database::models::audiobook::{AudiobookCreate, AudiobookUpdate};
use crate::database::models::genre::GenreSearch;
use crate::database::models::user::UserGetByUsername;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::genre::repository::GenreRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::{AppError, AppErrorKind};
use crate::forms::audiobook::{AudiobookCreateForm, AudiobookUploadForm};
use crate::templates::audiobook::{AudiobookCreateFormTemplate, AudiobookUploadFormTemplate};
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use uuid::Uuid;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use sqlx::postgres::types::PgInterval;

#[get("/create")]
pub async fn create_audiobook_form() -> Result<HttpResponse, AppError> {
    let template = AudiobookCreateFormTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/upload")]
pub async fn upload_audiobook_form() -> Result<HttpResponse, AppError> {
    let template = AudiobookUploadFormTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/create")]
pub async fn create_audiobook(
    identity: Option<Identity>,
    session: Session,
    audiobook_repo: web::Data<AudiobookRepository>,
    genre_repo: web::Data<GenreRepository>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<AudiobookCreateForm>,
) -> Result<HttpResponse, AppError> {
    let Some(u) = identity else {
        return Err(AppError::new(
            AppErrorKind::IdentityError,
            "Invalid identity",
        ));
    };
    let user = user_repo
        .read_one(&UserGetByUsername::new(&u.id()?))
        .await?;
    let genre_id = match genre_repo
        .read_many(&GenreSearch::new(&form.genre_name))
        .await?
        .first()
    {
        Some(g) => g.id,
        None => 1,
    };
    let book_crate = AudiobookCreate::new(
        &form.name,
        &user.id,
        &genre_id,
        &20,
        &0,
        &PgInterval {
            months: 0,
            days: 0,
            microseconds: 0,
        },
        "",
        &0,
        &0,
        "",
        "",
    );
    let book = audiobook_repo.create(&book_crate).await?;
    session.insert("audiobook_create_id", book.id)?;
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish())
}

#[post("/upload")]
pub async fn upload_audiobook(
    session: Session,
    audiobook_repo: web::Data<AudiobookRepository>,
    MultipartForm(form): MultipartForm<AudiobookUploadForm>,
) -> Result<HttpResponse, AppError> {

    // let path = format!("./media/thumbnails/audiobook_thumbnail_{}_{}", Uuid::new_v4(), form.thumbnail.file_name.unwrap());
    // log::info!("saving to {path}");
    let path = format!("./media/thumbnails/audiobook_thumbnail_{}_{}", Uuid::new_v4(), form.thumbnail.file_name.unwrap());
    log::info!("saving to {path}");
    let res = form.thumbnail.file.persist(path);
    match res {
        Ok(_) => {}
        Err(e) => {
            println!("Persist error {}", e);
            return Err(AppError::new(AppErrorKind::FileError, e.to_string().as_str()));
        }
    };


    let Some(book_id) = session.get::<i64>("audiobook_create_id")? else {
        return Err(AppError::new(AppErrorKind::NotFound, "Book could not be found in the active session"));
    };
    let book_update = AudiobookUpdate::new(
        &book_id,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None);
    let book = audiobook_repo.update(&book_update).await?;
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish())
}