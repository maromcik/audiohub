use crate::database::common::error::{BackendErrorKind, DbError};
use crate::templates::error::GenericError;
use actix_identity;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use askama::Template;
use serde::Serialize;
use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

/// User facing error type
#[derive(Error, Debug, Serialize, Clone)]
pub enum AppErrorKind {
    #[error("internal server error")]
    InternalServerError,
    #[error("not found")]
    NotFound,
    #[error("bad request")]
    BadRequest,
    #[error("templating error")]
    TemplatingError,
    #[error("identity error")]
    IdentityError,
    #[error("session error")]
    SessionError,
    #[error("password hasher error")]
    PasswordHasherError,
    #[error("conflict")]
    Conflict,
    #[error("file error")]
    FileError,
}

impl From<askama::Error> for AppError {
    fn from(_error: askama::Error) -> Self {
        Self::new(AppErrorKind::TemplatingError, "Templating error")
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AppError {
    pub error: AppErrorKind,
    pub message: String,
}

impl AppError {
    #[must_use]
    #[inline]
    pub fn new(error: AppErrorKind, description: &str) -> Self {
        Self {
            error,
            message: description.to_owned(),
        }
    }
}

impl From<DbError> for AppError {
    fn from(e: DbError) -> Self {
        match e.business_error.error_kind {
            BackendErrorKind::UserUpdateParametersEmpty
            | BackendErrorKind::AudiobookUpdateParametersEmpty
            | BackendErrorKind::ChapterUpdateParametersEmpty
            | BackendErrorKind::RatingUpdateParametersEmpty
            | BackendErrorKind::AudiobookDeleted
            | BackendErrorKind::ChapterDeleted
            | BackendErrorKind::GenreDeleted
            | BackendErrorKind::RatingDeleted
            | BackendErrorKind::UserDeleted => {
                Self::new(AppErrorKind::BadRequest, &e.to_string())
            }

            BackendErrorKind::UserDoesNotExist
            | BackendErrorKind::AudiobookDoesNotExist
            | BackendErrorKind::ChapterDoesNotExist
            | BackendErrorKind::GenreDoesNotExist
            | BackendErrorKind::RatingDoesNotExist => {
                Self::new(AppErrorKind::NotFound, &e.to_string())
            }

            BackendErrorKind::UniqueConstraintError => {
                Self::new(AppErrorKind::Conflict, &e.to_string())
            }

            _ => Self::new(AppErrorKind::InternalServerError, &e.to_string()),
        }
    }
}

impl From<actix_identity::error::LoginError> for AppError {
    fn from(value: actix_identity::error::LoginError) -> Self {
        Self::new(AppErrorKind::IdentityError, value.to_string().as_str())
    }
}

impl From<actix_identity::error::GetIdentityError> for AppError {
    fn from(value: actix_identity::error::GetIdentityError) -> Self {
        Self::new(AppErrorKind::IdentityError, value.to_string().as_str())
    }
}

impl From<actix_session::SessionInsertError> for AppError {
    fn from(value: actix_session::SessionInsertError) -> Self {
        Self::new(AppErrorKind::SessionError, value.to_string().as_str())
    }
}

impl From<actix_session::SessionGetError> for AppError {
    fn from(value: actix_session::SessionGetError) -> Self {
        Self::new(AppErrorKind::SessionError, value.to_string().as_str())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error code: {}, Error message: {}",
            self.error, self.message
        )
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error {
            AppErrorKind::BadRequest => StatusCode::BAD_REQUEST,
            AppErrorKind::NotFound => StatusCode::NOT_FOUND,
            AppErrorKind::Conflict => StatusCode::CONFLICT,
            AppErrorKind::TemplatingError
            | AppErrorKind::InternalServerError
            | AppErrorKind::PasswordHasherError
            | AppErrorKind::IdentityError
            | AppErrorKind::SessionError
            | AppErrorKind::FileError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        render_template(self)
    }
}

fn render_template(error: &AppError) -> HttpResponse {
    let template = GenericError {
        code: error.status_code().to_string(),
        description: error.message.clone(),
    };
    let body = template.render().unwrap_or_default();
    HttpResponse::build(error.status_code()).body(body)
}
