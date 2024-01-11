use crate::database::common::error::{BusinessLogicErrorKind, DbError};
use crate::templates::error::GenericError;
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
    #[error("conflict")]
    Conflict,
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
            BusinessLogicErrorKind::UserUpdateParametersEmpty
            | BusinessLogicErrorKind::AudiobookUpdateParametersEmpty
            | BusinessLogicErrorKind::ChapterUpdateParametersEmpty
            | BusinessLogicErrorKind::RatingUpdateParametersEmpty
            | BusinessLogicErrorKind::AudiobookDeleted
            | BusinessLogicErrorKind::ChapterDeleted
            | BusinessLogicErrorKind::GenreDeleted
            | BusinessLogicErrorKind::RatingDeleted
            | BusinessLogicErrorKind::UserDeleted => {
                Self::new(AppErrorKind::BadRequest, &e.to_string())
            }

            BusinessLogicErrorKind::UserDoesNotExist
            | BusinessLogicErrorKind::AudiobookDoesNotExist
            | BusinessLogicErrorKind::ChapterDoesNotExist
            | BusinessLogicErrorKind::GenreDoesNotExist
            | BusinessLogicErrorKind::RatingDoesNotExist => {
                Self::new(AppErrorKind::NotFound, &e.to_string())
            }

            BusinessLogicErrorKind::UniqueConstraintError => {
                Self::new(AppErrorKind::Conflict, &e.to_string())
            }

            _ => Self::new(AppErrorKind::InternalServerError, &e.to_string()),
        }
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
            AppErrorKind::TemplatingError | AppErrorKind::InternalServerError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
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
