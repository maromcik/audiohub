use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
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