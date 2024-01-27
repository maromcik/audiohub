use crate::database::models::Id;
use chrono::{DateTime, Utc};
use crate::database::common::HasDeletedAt;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Genre {
    pub id: Id,
    pub name: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl HasDeletedAt for Genre {
    fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct GenreCreate {
    pub name: String,
}

impl GenreCreate {
    #[must_use]
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GenreSearch {
    pub name: Option<String>,
}

impl GenreSearch {
    #[must_use]
    #[inline]
    pub fn new(name: Option<&str>) -> Self {
        Self {
            name: name.map(|n| n.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenreUpdate {
    pub id: Id,
    pub name: Option<String>,
    pub color: Option<String>
}

impl GenreUpdate {
    pub fn new(id: &Id, name: Option<&str>, color: Option<&str>) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: *id,
            name: name.and_then(change_to_owned),
            color: color.and_then(change_to_owned)
        }
    }

    #[inline]
    #[must_use]
    pub const fn update_fields_none(&self) -> bool {
        self.name.is_none()
        && self.color.is_none()
    }
}

#[derive(Clone, Debug)]
pub struct GenreDelete {
    pub id: Id,
}

impl GenreDelete {
    #[must_use]
    #[inline]
    pub fn new(id: &Id) -> Self {
        Self { id: *id }
    }
}

/// Structure for specific genre access
#[derive(Debug, Clone)]
pub struct GenreGetById {
    pub id: Id,
}

impl GenreGetById {
    #[must_use]
    #[inline]
    pub fn new(id: &Id) -> Self {
        Self { id: *id }
    }
}
