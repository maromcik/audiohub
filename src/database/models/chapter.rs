use chrono::{DateTime, Duration, Utc};
use sqlx::postgres::types::PgInterval;
use crate::database::models::Id;

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

#[derive(Debug, Clone)]
pub struct ChapterCreate {
    pub name: String,
    pub audiobook_id: Id,
    pub length: Duration,
    pub sequential_number: i32,
}

#[derive(Debug, Clone)]
pub struct ChapterUpdate {
    pub id: Id,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ChapterGetById {
    pub id: Id
}