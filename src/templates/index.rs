use crate::database::models::audiobook::AudiobookDetail;
use askama::Template;
use crate::database::models::active_audiobook::ActiveAudiobookDetail;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub logged_in: bool,
    pub username: String,
    pub audiobooks: Vec<AudiobookDetail>,
    pub active_audiobooks: Vec<ActiveAudiobookDetail>,
}

#[derive(Template)]
#[template(path = "index_content.html")]
pub struct IndexContentTemplate {
    pub logged_in: bool,
    pub username: String,
    pub audiobooks: Vec<AudiobookDetail>,
    pub active_audiobooks: Vec<ActiveAudiobookDetail>,
}

pub struct IndexBase {
    pub logged_in: bool,
    pub username: String,
    pub audiobooks: Vec<AudiobookDetail>,
    pub active_audiobooks: Vec<ActiveAudiobookDetail>,
}

impl From<IndexBase> for IndexContentTemplate {
    fn from(value: IndexBase) -> Self {
        Self {
            logged_in: value.logged_in,
            username: value.username,
            audiobooks: value.audiobooks,
            active_audiobooks: value.active_audiobooks
        }
    }
}

impl From<IndexBase> for IndexTemplate {
    fn from(value: IndexBase) -> Self {
        Self {
            logged_in: value.logged_in,
            username: value.username,
            audiobooks: value.audiobooks,
            active_audiobooks: value.active_audiobooks
        }
    }
}