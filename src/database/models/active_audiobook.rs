use crate::database::models::Id;
use sqlx::postgres::types::PgInterval;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct ActiveAudiobook {
    pub user_id: Id,
    pub audiobook_id: Id,
    pub playback_chapter_id: Option<Id>,
    pub playback_position_in_chapter: Option<PgInterval>,
}
