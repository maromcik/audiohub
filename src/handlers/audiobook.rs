use crate::database::common::{DbCreate, DbReadMany, DbReadOne};
use crate::database::models::audiobook::{
    AudiobookCreate, AudiobookGetById, AudiobookMetadataForm,
};
use crate::database::models::genre::{GenreGetById, GenreSearch};
use crate::database::models::user::{User, UserGetById};
use crate::database::models::Id;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::genre::repository::GenreRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::{AppError, AppErrorKind};
use crate::forms::audiobook::{AudiobookCreateForm, AudiobookUploadForm};
use crate::handlers::utilities::parse_user_id;
use crate::templates::audiobook::{
    AudiobookCreateFormTemplate, AudiobookDetailOwnerTemplate, AudiobookUploadFormTemplate,
};
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use sqlx::postgres::types::PgInterval;
use uuid::Uuid;

#[get("/create")]
pub async fn create_audiobook_form(
    genre_repo: web::Data<GenreRepository>,
) -> Result<HttpResponse, AppError> {
    let genres = genre_repo.read_many(&GenreSearch::new(None)).await?;

    let template = AudiobookCreateFormTemplate { genres };
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
    genre_repo: web::Data<GenreRepository>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<AudiobookCreateForm>,
) -> Result<HttpResponse, AppError> {
    let user = get_user_from_identity(identity, user_repo).await?;
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
    let user = get_user_from_identity(identity, user_repo).await?;
    let session_keys = AudiobookCreateSessionKeys::new(user.id);
    let thumbnail_path = format!(
        "./media/audiobook_{}_thumbnail_{}",
        uuid.clone(),
        form.thumbnail.file_name.unwrap_or_default()
    );

    let audiobook_path = format!(
        "./media/audiobook_{}_audio_{}",
        uuid.clone(),
        form.audio_file.file_name.unwrap_or_default()
    );

    let Some(thumbnail_mime) = form.thumbnail.content_type else {
        return Err(AppError::new(
            AppErrorKind::FileError,
            "No thumbnail MIME type found",
        ));
    };

    let Some(audiobook_mime) = form.audio_file.content_type else {
        return Err(AppError::new(
            AppErrorKind::FileError,
            "No audiobook MIME type found",
        ));
    };

    if !thumbnail_mime.to_string().starts_with("image/") {
        return Err(AppError::new(
            AppErrorKind::FileError,
            "Invalid thumbnail content type",
        ));
    }

    if !audiobook_mime.to_string().starts_with("audio/") {
        return Err(AppError::new(
            AppErrorKind::FileError,
            "Invalid audiobook content type",
        ));
    }

    let metadata = get_metadata_from_session(&session, &session_keys)?;

    let book_crate = AudiobookCreate::new(
        &metadata.name,
        &user.id,
        &metadata.genre_id,
        &PgInterval {
            months: 0,
            days: 0,
            microseconds: 0,
        },
        &audiobook_path,
        &thumbnail_path,
        &metadata.description,
    );
    let book = audiobook_repo.create(&book_crate).await?;

    log::info!("saving a thumbnail to {thumbnail_path}");
    if let Err(e) = form.thumbnail.file.persist(&thumbnail_path) {
        return Err(AppError::new(
            AppErrorKind::FileError,
            e.to_string().as_str(),
        ));
    };

    log::info!("saving an audiobook to {audiobook_path}");
    if let Err(e) = form.audio_file.file.persist(&audiobook_path) {
        return Err(AppError::new(
            AppErrorKind::FileError,
            e.to_string().as_str(),
        ));
    };

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
    _identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let audiobook = audiobook_repo
        .read_one(&AudiobookGetById::new(&path.into_inner().0))
        .await?;
    let template = AudiobookDetailOwnerTemplate { audiobook };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

fn get_metadata_from_session(
    session: &Session,
    session_keys: &AudiobookCreateSessionKeys,
) -> Result<AudiobookMetadataForm, AppError> {
    let Some(name) = session.get::<String>(session_keys.name.as_str())? else {
        return Err(AppError::new(
            AppErrorKind::NotFound,
            "New book could not be found in the active session",
        ));
    };

    let Some(genre_id) = session.get::<i64>(session_keys.genre_id.as_str())? else {
        return Err(AppError::new(
            AppErrorKind::NotFound,
            "New book could not be found in the active session",
        ));
    };

    let Some(description) = session.get::<String>(session_keys.description.as_str())? else {
        return Err(AppError::new(
            AppErrorKind::NotFound,
            "New book could not be found in the active session",
        ));
    };

    Ok(AudiobookMetadataForm {
        name,
        description,
        genre_id,
    })
}

async fn get_user_from_identity(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
) -> Result<User, AppError> {
    let Some(u) = identity else {
        return Err(AppError::new(
            AppErrorKind::IdentityError,
            "User must be logged in to upload a book",
        ));
    };
    Ok(user_repo
        .read_one(&UserGetById::new(&parse_user_id(u)?))
        .await?)
}

struct AudiobookCreateSessionKeys {
    name: String,
    description: String,
    genre_id: String,
}

impl AudiobookCreateSessionKeys {
    fn new(user_id: Id) -> Self {
        Self {
            name: format!("audiobook_create_{}_name", user_id),
            description: format!("audiobook_create_{}_description", user_id),
            genre_id: format!("audiobook_create_{}_genre_id", user_id),
        }
    }
}
