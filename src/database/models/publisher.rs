use crate::database::models::Id;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Publisher {
    pub id: Id,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct PublisherCreate {
    pub name: String,
}

impl PublisherCreate {
    #[must_use]
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PublisherSearch {
    pub name: String,
}

impl PublisherSearch {
    #[must_use]
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PublisherUpdate {
    pub id: Id,
    pub name: Option<String>,
}

impl PublisherUpdate {
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
pub struct PublisherDelete {
    pub id: Id,
}

impl PublisherDelete {
    #[must_use]
    #[inline]
    pub fn new(id: &Id) -> Self {
        Self { id: *id }
    }
}

/// Structure for specific publisher access
#[derive(Debug, Clone)]
pub struct PublisherGetById {
    pub id: Id,
}

impl PublisherGetById {
    #[must_use]
    #[inline]
    pub fn new(id: &Id) -> Self {
        Self { id: *id }
    }
}
