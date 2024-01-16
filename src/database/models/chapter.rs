use crate::database::models::Id;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::postgres::types::PgInterval;
use crate::database::models::genre::GenreSearch;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Chapter {
    pub id: Id,
    pub name: String,
    pub audiobook_id: Id,
    pub length: PgInterval,
    pub sequential_number: i32,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ChapterSearch {
    pub name: Option<String>,
    pub audiobook_id: Option<Id>,
    pub sequential_number: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChaptersGetByBookId {
    pub audiobook_id: Id,
}

#[derive(Debug, Clone)]
pub struct ChapterCreate {
    pub name: String,
    pub audiobook_id: Id,
    pub length: PgInterval,
    pub sequential_number: i32,
}

impl ChapterCreate {
    pub fn new(
        name: &str,
        audiobook_id: &Id,
        length: &PgInterval,
        sequential_number: &i32
    ) -> Self {
        Self {
            name: name.to_owned(),
            audiobook_id: *audiobook_id,
            length: length.to_owned(),
            sequential_number: *sequential_number,
        }
    }
}

impl ChapterSearch {
    pub fn new(
        name: Option<&str>,
        audiobook_id: Option<&Id>,
        sequential_number: Option<&i32>
    ) -> Self {
        Self {
            name: name.map(|n| n.to_owned()),
            audiobook_id: audiobook_id.map(|n| n.to_owned()),
            sequential_number: sequential_number.map(|n| n.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChapterUpdate {
    pub id: Id,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ChapterGetById {
    pub id: Id,
}
