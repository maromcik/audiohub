use crate::database::common::{DbCreate, DbDelete, DbReadMany, DbReadOne, DbUpdate};
use crate::database::models::audiobook::{AudiobookCreate, AudiobookDelete, AudiobookGetByIdJoin, AudiobookSearch, AudiobookUpdate};
use crate::database::models::genre::{GenreGetById, GenreSearch};

use crate::database::models::Id;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::database::repositories::genre::repository::GenreRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::forms::audiobook::{AudiobookCreateForm, AudiobookUploadForm};
use crate::handlers::utilities::{
    get_metadata_from_session, get_user_from_identity, remove_file, save_file, validate_file,
    AudiobookCreateSessionKeys,
};
use crate::templates::audiobook::{AudiobookCreateFormTemplate, AudiobookDetailCreatorTemplate, AudiobookDetailPageTemplate, AudiobookDetailVisitorTemplate, AudiobookUploadFormTemplate, NewReleasesTemplate};
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, HttpResponse, patch};
use askama::Template;

use crate::authorized;
use crate::database::common::error::{BackendError, BackendErrorKind};
use crate::database::common::query_parameters::DbQueryParams;
use crate::database::models::chapter::ChaptersGetByBookId;
use uuid::Uuid;
use crate::database::models::bookmark::BookmarkOperation;
use crate::database::models::user::UserGetById;
use crate::handlers::user_login;

#[get("/create")]
pub async fn create_audiobook_form(
    identity: Option<Identity>,
    genre_repo: web::Data<GenreRepository>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let genres = genre_repo.read_many(&GenreSearch::new(None)).await?;
    let template = AudiobookCreateFormTemplate { genres };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/upload")]
pub async fn upload_audiobook_form(identity: Option<Identity>) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let template = AudiobookUploadFormTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/create")]
pub async fn create_audiobook(
    identity: Option<Identity>,
    session: Session,
    genre_repo: web::Data<GenreRepository>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<AudiobookCreateForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity);
    let user = get_user_from_identity(u, &user_repo).await?;
    let session_keys = AudiobookCreateSessionKeys::new(user.id);
    let genre = genre_repo
        .read_one(&GenreGetById::new(&form.genre_id))
        .await?;

    session.insert(session_keys.name.as_str(), &form.name)?;
    session.insert(session_keys.genre_id.as_str(), genre.id)?;
    session.insert(session_keys.description.as_str(), &form.description)?;
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/audiobook/upload"))
        .finish())
}

#[post("/upload")]
pub async fn upload_audiobook(
    identity: Option<Identity>,
    session: Session,
    user_repo: web::Data<UserRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    MultipartForm(form): MultipartForm<AudiobookUploadForm>,
) -> Result<HttpResponse, AppError> {
    let uuid = Uuid::new_v4();
    let u = authorized!(identity);
    let user = get_user_from_identity(u, &user_repo).await?;
    let session_keys = AudiobookCreateSessionKeys::new(user.id);
    let thumbnail_path = validate_file(&form.thumbnail, uuid, "image", "audiobook")?;
    let audiobook_path = validate_file(&form.audio_file, uuid, "audio", "audiobook")?;

    let metadata = get_metadata_from_session(&session, &session_keys)?;

    let book_crate = AudiobookCreate::new(
        &metadata.name,
        &user.id,
        &metadata.genre_id,
        &audiobook_path,
        &thumbnail_path,
        &metadata.description,
    );
    let book = audiobook_repo.create(&book_crate).await?;

    save_file(form.thumbnail, thumbnail_path)?;
    save_file(form.audio_file, audiobook_path)?;

    session.remove(session_keys.name.as_str());
    session.remove(session_keys.description.as_str());
    session.remove(session_keys.genre_id.as_str());

    let handler = format!("/audiobook/{}/detail", book.id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, handler))
        .finish())
}

#[get("/{id}/detail")]
pub async fn get_audiobook(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    chapter_repo: web::Data<ChapterRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let user = get_user_from_identity(identity, &user_repo).await?;
    let book_id = path.into_inner().0;
    let audiobook = audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(&book_id))
        .await?;

    let chapters = chapter_repo
        .read_many(&ChaptersGetByBookId {
            audiobook_id: book_id,
        })
        .await?;

    // let body = match audiobook.author_id == user.id {
    //     true => (AudiobookDetailCreatorTemplate {
    //         audiobook,
    //         chapters,
    //     })
    //     .render()?,
    //     false => (AudiobookDetailVisitorTemplate {
    //         audiobook,
    //         chapters,
    //     })
    //     .render()?,
    // };

    let body = AudiobookDetailPageTemplate{audiobook}.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/releases")]
async fn releases(
    identity: Option<Identity>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    //add functionality for ordering audiobooks
    authorized!(identity);
    let books = book_repo
        .read_many(&AudiobookSearch::with_params(DbQueryParams::limit(5, 0)))
        .await?;

    let template = NewReleasesTemplate { audiobooks: books };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}/delete")]
pub async fn remove_audiobook(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let user = get_user_from_identity(identity, &user_repo).await?;
    let audiobook = audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(&path.into_inner().0))
        .await?;

    if user.id != audiobook.author_id {
        return Err(AppError::from(BackendError::new(
            BackendErrorKind::UnauthorizedOperation,
        )));
    }
    remove_file(&audiobook.file_path)?;
    remove_file(&audiobook.thumbnail)?;
    audiobook_repo
        .delete(&AudiobookDelete::new(&audiobook.id))
        .await?;
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish())
}

#[patch("{id}/likes")]
pub async fn change_like(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    audiobook_repo: web::Data<AudiobookRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {

    let identity = authorized!(identity);

    let user = get_user_from_identity(identity, &user_repo).await?;
    let user_get_by_id = UserGetById{id: user.id};
    let bookmarks = &user_repo.get_all_bookmarks(&user_get_by_id).await?;

    let liked = bookmarks.iter().any(|bookmark| bookmark.user_id == user.id);

    let book_id = path.into_inner().0;
    let audiobook = audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(&book_id))
        .await?;

    let bookmark = BookmarkOperation{user_id: user.id, audiobook_id: book_id };
    let likes = match liked {
        true => {
            user_repo.unbookmark(&bookmark).await?;
            audiobook.like_count - 1
        },
        false => {
            user_repo.bookmark(&bookmark).await?;
            audiobook.like_count + 1
        }
    };

    let update = AudiobookUpdate{id: book_id.clone(), author_id: None, genre_id: None, name: None, description: None, file_path: None, overall_rating: None, thumbnail: None, stream_count: None, like_count: Some(likes)};

    audiobook_repo.update(&update).await?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(likes.to_string()))
}


