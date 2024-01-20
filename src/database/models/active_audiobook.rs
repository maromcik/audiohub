use crate::database::models::Id;
use chrono::{DateTime, Utc};
use crate::CONSIDER_AUDIOBOOK_FINISHED_PERCENTAGE;
use crate::database::models::audiobook::AudiobookDetail;

#[derive(sqlx::FromRow, Debug, PartialEq, Clone)]
pub struct ActiveAudiobook {
    pub user_id: Id,
    pub audiobook_id: Id,
    pub playback_position: f64,
    pub edited_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct RemoveActiveAudiobook {
    pub user_id: Id,
    pub audiobook_id: Id,
}

impl RemoveActiveAudiobook {
    #[must_use]
    #[inline]
    pub const fn new(user_id: Id, audiobook_id: Id) -> Self {
        Self {
            user_id,
            audiobook_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SetActiveAudiobook {
    pub user_id: Id,
    pub audiobook_id: Id,
    pub playback_position: f64,
}


#[derive(Debug, Clone)]
pub struct PlayedAudiobook {
    pub book_id: Id,
    pub author_id: Id,
    pub author_name: String,
    pub author_surname: String,
    pub is_liked: Option<bool>,
    pub path: String,
    pub name: String,
    pub thumbnail: String,
    pub playback_position: f64,
}

impl SetActiveAudiobook {
    #[must_use]
    #[inline]
    pub const fn new(user_id: Id, audiobook_id: Id, playback_position: f64) -> Self {
        Self {
            user_id,
            audiobook_id,
            playback_position,
        }
    }
}
