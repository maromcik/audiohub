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

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct ActiveAudiobookJoin {
    pub id: Id,
    // --------------
    pub name: String,
    pub author_id: Id,
    pub genre_id: Id,
    pub file_path: String,
    pub length: f64,
    pub stream_count: i64,
    pub like_count: i64,
    pub overall_rating: i16,
    pub thumbnail: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,

    pub username: String,
    pub email: String,
    pub author_name: String,
    pub surname: String,
    pub bio: String,
    pub profile_picture: String,

    pub genre_name: String,

    pub playback_position: f64,
    pub active_audiobook_edited_at: DateTime<Utc>,
}

impl ActiveAudiobookJoin {
    pub fn is_finished(&self) -> bool {
        self.playback_position / self.length * 100f64 > CONSIDER_AUDIOBOOK_FINISHED_PERCENTAGE
    }
}


#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct ActiveAudiobookDetail {
    pub id: Id,
    // --------------
    pub name: String,
    pub author_id: Id,
    pub genre_id: Id,
    pub file_path: String,
    pub length: f64,
    pub stream_count: i64,
    pub like_count: i64,
    pub overall_rating: i16,
    pub thumbnail: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,

    pub username: String,
    pub email: String,
    pub author_name: String,
    pub surname: String,
    pub bio: String,
    pub profile_picture: String,

    pub genre_name: String,

    pub playback_position: f64,
    pub active_audiobook_edited_at: DateTime<Utc>,
    pub progress: f64
}

impl ActiveAudiobookDetail {
    pub fn from_audiobook(audiobook: &AudiobookDetail, playback_position: f64, active_audiobook_edited_at: DateTime<Utc>) -> Self {
        Self {
            id: audiobook.id,
            name: audiobook.name.to_owned(),
            author_id: audiobook.author_id,
            genre_id: audiobook.genre_id,
            file_path: audiobook.file_path.to_owned(),
            length: audiobook.length,
            thumbnail: audiobook.thumbnail.to_owned(),
            description: audiobook.description.to_owned(),
            stream_count: audiobook.stream_count,
            like_count: audiobook.like_count,
            overall_rating: audiobook.overall_rating,
            created_at: audiobook.created_at,
            edited_at: audiobook.edited_at,

            username: audiobook.username.to_owned(),
            email: audiobook.email.to_owned(),
            author_name: audiobook.author_name.to_owned(),
            surname: audiobook.surname.to_owned(),
            bio: audiobook.bio.to_owned(),
            profile_picture: audiobook.profile_picture.to_owned(),
            genre_name: audiobook.genre_name.to_owned(),

            playback_position,
            active_audiobook_edited_at,
            progress: playback_position / audiobook.length * 100f64
        }
    }
    pub fn is_finished(&self) -> bool {
        self.playback_position / self.length * 100f64 > CONSIDER_AUDIOBOOK_FINISHED_PERCENTAGE
    }
}

impl From<ActiveAudiobookJoin> for ActiveAudiobookDetail {
    fn from(audiobook: ActiveAudiobookJoin) -> Self {
        Self {
            id: audiobook.id,
            name: audiobook.name.to_owned(),
            author_id: audiobook.author_id,
            genre_id: audiobook.genre_id,
            file_path: audiobook.file_path.to_owned(),
            length: audiobook.length,
            thumbnail: audiobook.thumbnail.to_owned(),
            description: audiobook.description.to_owned(),
            stream_count: audiobook.stream_count,
            like_count: audiobook.like_count,
            overall_rating: audiobook.overall_rating,
            created_at: audiobook.created_at,
            edited_at: audiobook.edited_at,

            username: audiobook.username.to_owned(),
            email: audiobook.email.to_owned(),
            author_name: audiobook.author_name.to_owned(),
            surname: audiobook.surname.to_owned(),
            bio: audiobook.bio.to_owned(),
            profile_picture: audiobook.profile_picture.to_owned(),
            genre_name: audiobook.genre_name.to_owned(),
            playback_position: audiobook.playback_position,
            active_audiobook_edited_at: audiobook.active_audiobook_edited_at,
            progress: audiobook.playback_position / audiobook.length * 100f64
        }
    }
}