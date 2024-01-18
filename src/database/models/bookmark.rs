use chrono::{DateTime, Utc};
use crate::database::models::Id;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Bookmark {
    pub user_id: Id,
    pub audiobook_id: Id,
    pub edited_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct BookmarkOperation {
    pub user_id: Id,
    pub audiobook_id: Id,
}

impl BookmarkOperation {
    #[must_use]
    #[inline]
    pub const fn new(user_id: Id, audiobook_id: Id) -> Self {
        Self {
            user_id,
            audiobook_id
        }
    }
}
