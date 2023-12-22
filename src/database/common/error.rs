use std::fmt::{Debug, Display, Formatter};

use BusinessLogicErrorKind::*;

#[derive(Debug)]
pub enum BusinessLogicErrorKind {
    // User errors
    UserDoesNotExist,
    UserDeleted,
    UserPasswordDoesNotMatch,
    UserUpdateParametersEmpty,

    // --------------------------
    // Publisher errors
    PublisherDoesNotExist,
    PublisherDeleted,
    PublisherUpdateParametersEmpty,

    // Audiobook errors
    AudiobookDoesNotExist,
    AudiobookDeleted,
    AudiobookUpdateParametersEmpty,

    // --------------------------
    // Rating errors
    RatingDoesNotExist,
    RatingDeleted,
    RatingUpdateEmpty,

    // --------------------------
    // Chapter errors
    ChapterDoesNotExist,
    ChapterDeleted,
    ChapterUpdateEmpty,

    // ------------------
    // Author errors
    AuthorDoesNotExist,
    AuthorDeleted,
    AuthorUpdateParametersEmpty
}

impl Display for BusinessLogicErrorKind {
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
            RatingDoesNotExist => f.write_str(does_not_exist("rating").as_str()),
            RatingDeleted => f.write_str(deleted("rating").as_str()),
            RatingUpdateEmpty => {
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
            ChapterUpdateEmpty => {
                write!(
                    f,
                    concat!(
                    "The provided parameters for Chapter update query are incorrect",
                    " (no Chapter field would be changed)."
                    )
                )
            }
            PublisherDoesNotExist => f.write_str(does_not_exist("publisher").as_str()),
            PublisherDeleted => f.write_str(deleted("publisher").as_str()),
            PublisherUpdateParametersEmpty => {
                write!(
                    f,
                    concat!(
                        "The provided parameters for Publisher update query are incorrect",
                        " (no Publisher field would be changed)."
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
        }
    }
}

/// Error type representing a Business Logic Error in the database layer ->
/// usually a problem with missing records, insufficient rights for operation, and so on.
pub struct BusinessLogicError {
    error: BusinessLogicErrorKind,
}

impl BusinessLogicError {
    /// Business Logic Error constructor
    #[must_use]
    #[inline]
    pub const fn new(error: BusinessLogicErrorKind) -> Self {
        Self { error }
    }

    /// Formatted business logic error
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Business logic error: {}", self.error)
    }
}

impl Display for BusinessLogicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for BusinessLogicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

pub struct DbError {
    description: String,
}

/// Error encapsulating errors from `sqlx` and our own `BusinessLogicError`, unifying errors from
/// the database without the need of `anyhow` library.
impl DbError {
    /// Database Error constructor
    #[must_use]
    #[inline]
    pub fn new(description: &str) -> Self {
        Self {
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
impl From<sqlx::Error> for DbError {
    fn from(value: sqlx::Error) -> Self {
        Self::new(&format!("sqlx error: {value}"))
    }
}

/// Conversion from sqlx migrate error, useful when using `?` operator
impl From<sqlx::migrate::MigrateError> for DbError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::new(&format!("Migration error: {value}"))
    }
}

/// Conversion from business logic error
impl From<BusinessLogicError> for DbError {
    fn from(value: BusinessLogicError) -> Self {
        Self::new(value.to_string().as_str())
    }
}

/// generic database result
pub type DbResult<T> = Result<T, DbError>;

/// Syntax sugar type denoting a singular result from the database
pub type DbResultSingle<T> = DbResult<T>;
/// Syntax sugar type denoting multiple results from the database
pub type DbResultMultiple<T> = DbResult<Vec<T>>;
