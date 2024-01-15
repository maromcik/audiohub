use crate::database::models::Id;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::postgres::types::PgInterval;

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

#[derive(Debug, Clone)]
pub struct ChapterUpdate {
    pub id: Id,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ChapterGetById {
    pub id: Id,
}
