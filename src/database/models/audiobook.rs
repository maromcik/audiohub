use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Audiobook {
    pub id: Uuid,
    // --------------
    pub name: String,
    pub author_id: Uuid,
    pub publisher_id: Uuid,
    pub genre_id: Uuid,
    pub price_dollars: u32,
    pub price_cents: u32,
    pub file_path: String,
    pub stream_count: u64,
    pub overall_rating: u8,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
