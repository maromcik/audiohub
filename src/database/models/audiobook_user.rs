use chrono::Duration;
use crate::database::models::Id;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct AudiobookUser {
    pub user_id: Id,
    pub audiobook_id: Id,
    pub playback_chapter_id: Id,
    pub playback_position_in_chapter: Option<Duration>,
}
