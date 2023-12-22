use crate::database::models::Id;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Author {
    pub id: Id,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct AuthorCreate {
    pub name: String,
}

impl AuthorCreate {
    #[must_use]
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthorUpdate {
    pub id: Id,
    pub name: Option<String>,
}

impl AuthorUpdate {
    pub fn new(id: &Id, name: Option<&str>) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: *id,
            name: name.and_then(change_to_owned),
        }
    }

    #[inline]
    #[must_use]
    pub const fn update_fields_none(&self) -> bool {
        self.name.is_none()
    }
}

#[derive(Clone, Debug)]
pub struct AuthorDelete {
    pub id: Id,
}

impl AuthorDelete {
    #[must_use]
    #[inline]
    pub fn new(id: &Id) -> Self {
        Self { id: *id }
    }
}

/// Structure for specific author access
#[derive(Debug, Clone)]
pub struct AuthorGetById {
    pub id: Id,
}

impl AuthorGetById {
    #[must_use]
    #[inline]
    pub fn new(id: &Id) -> Self {
        Self { id: *id }
    }
}
