use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Rating {
    pub id: Uuid,
    pub audiobook_id: Uuid,
    pub user_id: Uuid,
    pub rating: u8,
    pub review: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}