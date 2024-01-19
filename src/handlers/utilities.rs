use crate::database::common::DbReadOne;
use crate::database::models::audiobook::{ActiveAudiobookDetail, AudiobookDetail, AudiobookMetadataForm};
use crate::database::models::user::{User, UserGetById};
use crate::database::models::Id;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::{AppError, AppErrorKind};
use actix_identity::Identity;
use actix_multipart::form::tempfile::TempFile;
use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{web, HttpResponse};

use uuid::Uuid;

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
    identity: Identity,
    user_repo: &web::Data<UserRepository>,
) -> Result<User, AppError> {
    Ok(user_repo
        .read_one(&UserGetById::new(&parse_user_id(identity)?))
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

pub fn validate_file(
    file: &TempFile,
    uuid: Uuid,
    mime: &str,
    handler: &str,
) -> Result<String, AppError> {
    let extension = match file.file_name.clone() {
        None => "".to_owned(),
        Some(name) => {
            let split_res = name.split('.');
            let vector = split_res.collect::<Vec<&str>>();
            match vector.last() {
                None => "".to_owned(),
                Some(ext) => ext.to_string(),
            }
        }
    };
    let file_path = format!("/media/{handler}_{uuid}_{mime}.{extension}");

    let Some(file_mime) = &file.content_type else {
        return Err(AppError::new(
            AppErrorKind::FileError,
            "No thumbnail MIME type found",
        ));
    };

    if !file_mime
        .to_string()
        .starts_with(format!("{mime}/").as_str())
    {
        return Err(AppError::new(
            AppErrorKind::FileError,
            "Invalid thumbnail content type",
        ));
    }
    Ok(file_path)
}

pub fn save_file(file: TempFile, path: String) -> Result<(), AppError> {
    log::info!("saving file to .{path}");
    let path = format!(".{path}");
    if let Err(e) = file.file.persist(path) {
        return Err(AppError::new(
            AppErrorKind::FileError,
            e.to_string().as_str(),
        ));
    };
    Ok(())
}

pub fn remove_file(path: &str) -> Result<(), AppError> {
    if !path.is_empty() && std::path::Path::new(path).exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

pub fn get_active_audiobooks(audiobooks: &Vec<AudiobookDetail>) ->Vec<ActiveAudiobookDetail> {
    audiobooks
        .iter()
        .filter_map(|a| match (a.playback_position, a.active_audiobook_edited_at) {
            (Some(pos), Some(edited)) => if (a.length - pos) > 5f64 {
                Some(ActiveAudiobookDetail::from_audiobook(a, pos, edited))
            } else { None }
            (_, _) => None
        }).collect()
}

#[macro_export]
macro_rules! authorized {
    ($e:expr) => {
        match $e {
            None => {
                return Ok(HttpResponse::SeeOther()
                    .insert_header((LOCATION, "/user/login"))
                    .finish());
            }
            Some(v) => v,
        }
    };
}
