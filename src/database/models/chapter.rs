use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Chapter {
    pub id: Uuid,
    pub audiobook_id: Uuid,
    pub name: String,
    pub length: Duration,
    pub sequential_number: i32,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
