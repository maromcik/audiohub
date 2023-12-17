use chrono::{DateTime, Utc, Duration};
use crate::database::models::Id;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Audiobook {
    pub id: Id,
    // --------------
    pub name: String,
    pub author_id: Id,
    pub publisher_id: Id,
    pub genre_id: Id,
    pub price_dollars: i32,
    pub price_cents: i32,
    pub length: Duration,
    pub file_path: String,
    pub stream_count: i64,
    pub overall_rating: i16,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

pub struct AudioBookGetById {
    pub id: Id
}
