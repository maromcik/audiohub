use pbkdf2;
use std::fmt::{Debug, Display, Formatter};

use BackendErrorKind::*;

#[derive(Debug, Clone)]
pub enum BackendErrorKind {
    // User errors
    UserDoesNotExist,
    UserDeleted,
    UserPasswordDoesNotMatch,
    UserUpdateParametersEmpty,
    UserPasswordVerificationFailed,

    // Audiobook errors
    AudiobookDoesNotExist,
    AudiobookDeleted,
    AudiobookUpdateParametersEmpty,

    // --------------------------
    // Rating errors
    RatingDoesNotExist,
    RatingDeleted,
    RatingUpdateParametersEmpty,

    // --------------------------
    // Chapter errors
    ChapterDoesNotExist,
    ChapterDeleted,
    ChapterUpdateParametersEmpty,

    GenreDeleted,
    GenreDoesNotExist,
    GenreUpdateParametersEmpty,

    DatabaseError,
    MigrationError,
    UniqueConstraintError,
    NotNullError,
    ForeignKeyError,
}

impl Display for BackendErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let does_not_exist = |name: &str| format!("The specified {name} does not exist!");
        let deleted = |name: &str| format!("The specified {name} has been deleted!");

        match self {
            UserDoesNotExist => f.write_str(does_not_exist("user").as_str()),
            UserDeleted => f.write_str(deleted("user").as_str()),
            UserPasswordDoesNotMatch => {
                write!(
                    f,
                    "The provided email and password combination is incorrect."
                )
            }
            UserPasswordVerificationFailed => {
                write!(f, "Password verification failed.")
            }
            RatingDoesNotExist => f.write_str(does_not_exist("rating").as_str()),
            RatingDeleted => f.write_str(deleted("rating").as_str()),
            RatingUpdateParametersEmpty => {
                write!(
                    f,
                    concat!(
                        "The provided parameters for Rating update query are incorrect",
                        " (no Rating field would be changed)."
                    )
                )
            }
            ChapterDoesNotExist => f.write_str(does_not_exist("chapter").as_str()),
            ChapterDeleted => f.write_str(deleted("chapter").as_str()),
            ChapterUpdateParametersEmpty => {
                write!(
                    f,
                    concat!(
                        "The provided parameters for Chapter update query are incorrect",
                        " (no Chapter field would be changed)."
                    )
                )
            }
            AudiobookDoesNotExist => f.write_str(does_not_exist("audiobook").as_str()),
            AudiobookDeleted => f.write_str(deleted("audiobook").as_str()),
            AudiobookUpdateParametersEmpty => {
                write!(
                    f,
                    concat!(
                        "The provided parameters for Audiobook update query are incorrect",
                        " (no Audiobook field would be changed)."
                    )
                )
            }
            UserUpdateParametersEmpty => {
                write!(
                    f,
                    concat!(
                        "The provided parameters for User update query are incorrect",
                        " (no User field would be changed)."
                    )
                )
            }
            GenreDoesNotExist => f.write_str(does_not_exist("genre").as_str()),
            GenreDeleted => f.write_str(deleted("genre").as_str()),
            GenreUpdateParametersEmpty => {
                write!(
                    f,
                    concat!(
                        "The provided parameters for Genre update query are incorrect",
                        " (no Genre field would be changed)."
                    )
                )
            }
            DatabaseError => write!(f, "Unknown database error occured"),
            MigrationError => write!(f, "Unknown migration-related error occured"),
            UniqueConstraintError => write!(f, "Duplicate value"),
            NotNullError => write!(f, "Field cannot be null"),
            ForeignKeyError => write!(f, "Related field error"),
        }
    }
}

/// Error type representing a Business Logic Error in the database layer ->
/// usually a problem with missing records, insufficient rights for operation, and so on.
#[derive(Clone)]
pub struct BackendError {
    pub error_kind: BackendErrorKind,
}

impl BackendError {
    /// Business Logic Error constructor
    #[must_use]
    #[inline]
    pub const fn new(error: BackendErrorKind) -> Self {
        Self { error_kind: error }
    }

    /// Formatted business logic error
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Business logic error: {}", self.error_kind)
    }
}

impl Display for BackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for BackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

#[derive(Clone)]
pub struct DbError {
    pub business_error: BackendError,
    description: String,
}

/// Error encapsulating errors from `sqlx` and our own `BusinessLogicError`, unifying errors from
/// the database without the need of `anyhow` library.
impl DbError {
    /// Database Error constructor
    #[must_use]
    #[inline]
    pub fn new(error: BackendError, description: &str) -> Self {
        Self {
            business_error: error,
            description: description.to_owned(),
        }
    }
    /// Formatted database error
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Database Error] {}", self.description)
    }
}

impl Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

/// The database error can be assigned to `dyn Error`
impl std::error::Error for DbError {
    fn description(&self) -> &str {
        &self.description
    }
}

/// Conversion from sqlx error, useful when using `?` operator
// impl From<sqlx::Error> for DbError {
//     fn from(value: sqlx::Error) -> Self {
//         Self::new(BusinessLogicError::new(DatabaseError), &format!("sqlx error: {value}"))
//     }
// }

impl From<sqlx::Error> for DbError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(err) => match err.kind() {
                sqlx::error::ErrorKind::ForeignKeyViolation => Self::new(
                    BackendError::new(ForeignKeyError),
                    &format!("sqlx error: {err}"),
                ),
                sqlx::error::ErrorKind::UniqueViolation => Self::new(
                    BackendError::new(UniqueConstraintError),
                    &format!("sqlx error: {err}"),
                ),
                sqlx::error::ErrorKind::NotNullViolation => Self::new(
                    BackendError::new(NotNullError),
                    &format!("sqlx error: {err}"),
                ),
                _ => Self::new(
                    BackendError::new(DatabaseError),
                    &format!("sqlx error: {err}"),
                ),
            },
            _ => Self::new(
                BackendError::new(DatabaseError),
                &format!("sqlx error: {value}"),
            ),
        }
    }
}

/// Conversion from sqlx migrate error, useful when using `?` operator
impl From<sqlx::migrate::MigrateError> for DbError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::new(
            BackendError::new(MigrationError),
            &format!("Migration error: {value}"),
        )
    }
}

/// Conversion from business logic error
impl From<BackendError> for DbError {
    fn from(value: BackendError) -> Self {
        Self::new(value.clone(), value.to_string().as_str())
    }
}

impl From<pbkdf2::password_hash::Error> for DbError {
    fn from(value: pbkdf2::password_hash::Error) -> Self {
        Self::new(
            BackendError::new(UserPasswordVerificationFailed),
            value.to_string().as_str(),
        )
    }
}

/// generic database result
pub type DbResult<T> = Result<T, DbError>;

/// Syntax sugar type denoting a singular result from the database
pub type DbResultSingle<T> = DbResult<T>;
/// Syntax sugar type denoting multiple results from the database
pub type DbResultMultiple<T> = DbResult<Vec<T>>;
