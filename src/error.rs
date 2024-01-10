use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::database::common::error::{BusinessLogicError, BusinessLogicErrorKind, DbError};

#[derive(Serialize, Deserialize)]
struct Error {
    error: String,
}

impl From<askama::Error> for AppError {
    fn from(_error: askama::Error) -> Self {
        Self::TemplatingError
    }
}

/// User facing error type
#[derive(Error, Debug, Serialize)]
pub enum AppError {
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
            | BusinessLogicErrorKind::UserDeleted => Self::BadRequest,

            BusinessLogicErrorKind::UserDoesNotExist
            | BusinessLogicErrorKind::AudiobookDoesNotExist
            | BusinessLogicErrorKind::ChapterDoesNotExist
            | BusinessLogicErrorKind::GenreDoesNotExist
            | BusinessLogicErrorKind::RatingDoesNotExist => Self::NotFound,
            
            BusinessLogicErrorKind::UniqueConstraintError => Self::Conflict,

            _ => Self::InternalServerError
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::TemplatingError | AppError::InternalServerError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .json(Error {
                error: self.to_string(),
            })
    }
}
