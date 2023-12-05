use chrono::{DateTime, Utc};
use crate::database::models::Id;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Rating {
    pub id: Id,
    pub audiobook_id: Id,
    pub user_id: Id,
    pub rating: i16,
    pub review: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
