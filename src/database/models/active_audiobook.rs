use crate::database::models::Id;

#[derive(sqlx::FromRow, Debug, PartialEq, Clone)]
pub struct ActiveAudiobook {
    pub user_id: Id,
    pub audiobook_id: Id,
    pub playback_chapter_id: Id,
    pub playback_position_in_chapter: f64,
}

#[derive(Debug, Clone)]
pub struct AddActiveAudiobook {
    pub user_id: Id,
    pub audiobook_id: Id,
    pub playback_chapter_id: Id,
    pub playback_position_in_chapter: Option<f64>,
}

impl AddActiveAudiobook {
    #[must_use]
    #[inline]
    pub const fn new(
        user_id: Id,
        audiobook_id: Id,
        playback_chapter_id: Id,
        playback_position_in_chapter: Option<f64>,
    ) -> Self {
        Self {
            user_id,
            audiobook_id,
            playback_chapter_id,
            playback_position_in_chapter,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RemoveActiveAudiobook {
    pub user_id: Id,
    pub audiobook_id: Id,
    pub playback_chapter_id: Id,
}

impl RemoveActiveAudiobook {
    #[must_use]
    #[inline]
    pub const fn new(user_id: Id, audiobook_id: Id, playback_chapter_id: Id) -> Self {
        Self {
            user_id,
            audiobook_id,
            playback_chapter_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SetActiveAudiobook {
    pub user_id: Id,
    pub audiobook_id: Id,
    pub playback_chapter_id: Id,
    pub playback_position_in_chapter: f64,
}

impl SetActiveAudiobook {
    #[must_use]
    #[inline]
    pub const fn new(
        user_id: Id,
        audiobook_id: Id,
        playback_chapter_id: Id,
        playback_position_in_chapter: f64,
    ) -> Self {
        Self {
            user_id,
            audiobook_id,
            playback_chapter_id,
            playback_position_in_chapter,
        }
    }
}