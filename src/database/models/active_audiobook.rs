use crate::database::models::Id;
use chrono::{DateTime, Utc};

use crate::database::models::utilities::get_default_thumbnail;

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
    #[allow(dead_code)]
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
pub struct PlayedAudiobookDb {
    pub book_id: Id,
    pub author_id: Id,
    pub author_name: String,
    pub author_surname: String,
    pub is_liked: Option<bool>,
    pub path: String,
    pub name: String,
    pub thumbnail: Option<String>,
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

impl From<PlayedAudiobookDb> for PlayedAudiobook {
    fn from(value: PlayedAudiobookDb) -> Self {
        Self {
            book_id: value.book_id,
            author_id: value.author_id,
            author_name: value.author_name,
            author_surname: value.author_surname,
            is_liked: value.is_liked,
            path: value.path,
            name: value.name,
            thumbnail: get_default_thumbnail(&value.thumbnail),
            playback_position: value.playback_position,
        }
    }
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
