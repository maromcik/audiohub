use crate::database::common::DbReadOne;
use crate::database::models::audiobook::AudiobookMetadataForm;
use crate::database::models::user::{User, UserGetById};
use crate::database::models::Id;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::{AppError, AppErrorKind};
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use actix_web::http::header::LOCATION;

pub fn parse_user_id(identity: Identity) -> Result<Id, AppError> {
    Ok(identity.id()?.parse::<i64>()?)
}

pub fn is_unauthorized(identity: Option<Identity>) -> bool {
    identity.is_none()
}

pub fn redirect_login() -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/user/login"))
        .finish()
}

pub fn get_metadata_from_session(
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

pub async fn get_user_from_identity(
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

pub struct AudiobookCreateSessionKeys {
    pub name: String,
    pub description: String,
    pub genre_id: String,
}

impl AudiobookCreateSessionKeys {
    pub fn new(user_id: Id) -> Self {
        Self {
            name: format!("audiobook_create_{}_name", user_id),
            description: format!("audiobook_create_{}_description", user_id),
            genre_id: format!("audiobook_create_{}_genre_id", user_id),
        }
    }
}


#[macro_export]
macro_rules! authorized {
    ($e:expr) => {
        match $e {
            None => {
                return Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/user/login"))
        .finish()); }
            Some(v) => {v}
        }
    };
}