use crate::database::models::Id;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Rating {
    pub id: Id,
    pub audiobook_id: Id,
    pub user_id: Id,
    pub rating: i16,
    pub review: Option<String>,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct RatingCreate {
    pub audiobook_id: Id,
    pub user_id: Id,
    pub rating: i16,
    pub review: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RatingUpdate {
    pub id: Id,
    pub rating: i16,
    pub review: Option<String>,
}

pub struct RatingGetById {
    pub id: Id,
}
