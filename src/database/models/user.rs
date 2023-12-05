use chrono::{DateTime, Utc};
use crate::database::models::Id;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Id,
    // --------------
    pub username: String,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub bio: String,
    pub profile_picture: String,
    pub password_hash: String,
    pub password_salt: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
