use crate::database::models::Id;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(sqlx::FromRow, Debug, PartialEq, Clone)]
pub struct Chapter {
    pub id: Id,
    pub name: String,
    pub audiobook_id: Id,
    pub position: f64,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}


#[derive(sqlx::FromRow, Debug, PartialEq, Clone)]
pub struct ChapterDetail {
    pub id: Id,
    pub name: String,
    pub audiobook_id: Id,
    pub position: f64,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub audiobook_name: String,
    pub author_id: Id,
}


#[derive(Debug, Clone, Deserialize, Default)]
pub struct ChapterSearch {
    pub name: Option<String>,
    pub audiobook_id: Option<Id>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChaptersGetByBookId {
    pub audiobook_id: Id,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChaptersGetByBookIdJoin {
    pub audiobook_id: Id,
}

impl ChaptersGetByBookId {
    pub fn new(id: Id) -> Self {
        Self { audiobook_id: id }
    }
}

#[derive(Debug, Clone)]
pub struct ChapterDisplay {
    pub id: Id,
    pub name: String,
    pub position: f64,
    pub order: usize,
}
#[derive(Debug, Clone)]
pub struct ChapterCreate {
    pub name: String,
    pub audiobook_id: Id,
    pub position: f64,
}

impl ChapterCreate {
    pub fn new(name: &str, audiobook_id: &Id, position: &f64) -> Self {
        Self {
            name: name.to_owned(),
            audiobook_id: *audiobook_id,
            position: *position,
        }
    }
}

impl ChapterSearch {
    pub fn new(name: Option<&str>, audiobook_id: Option<&Id>) -> Self {
        Self {
            name: name.map(|n| n.to_owned()),
            audiobook_id: audiobook_id.map(|n| n.to_owned()),
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

impl ChapterGetById {
    pub fn new(id: Id) -> Self {
        Self {
            id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChapterRemoveById {
    pub id: Id,
    pub user_id: Id
}

impl ChapterRemoveById {
    pub fn new(id: Id, user_id: Id) -> Self {
        Self {
            id,
            user_id
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateChapterForm {
    pub book_id: Id,
    pub current_chapters: Vec<ChapterDisplay>,
    pub book_filepath: String,
}