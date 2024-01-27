use crate::database::models::audiobook::AudiobookDisplay;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub logged_in: bool,
    pub username: String,
    pub audiobooks: Vec<AudiobookDisplay>,
    pub active_audiobooks: Vec<AudiobookDisplay>,
    pub finished_audiobooks: Vec<AudiobookDisplay>,
}

#[derive(Template)]
#[template(path = "index_content.html")]
pub struct IndexContentTemplate {
    pub logged_in: bool,
    pub username: String,
    pub audiobooks: Vec<AudiobookDisplay>,
    pub active_audiobooks: Vec<AudiobookDisplay>,
    pub finished_audiobooks: Vec<AudiobookDisplay>,
}

pub struct IndexBase {
    pub logged_in: bool,
    pub username: String,
    pub audiobooks: Vec<AudiobookDisplay>,
    pub active_audiobooks: Vec<AudiobookDisplay>,
    pub finished_audiobooks: Vec<AudiobookDisplay>,
}

impl From<IndexBase> for IndexContentTemplate {
    fn from(value: IndexBase) -> Self {
        Self {
            logged_in: value.logged_in,
            username: value.username,
            audiobooks: value.audiobooks,
            active_audiobooks: value.active_audiobooks,
            finished_audiobooks: value.finished_audiobooks,
        }
    }
}

impl From<IndexBase> for IndexTemplate {
    fn from(value: IndexBase) -> Self {
        Self {
            logged_in: value.logged_in,
            username: value.username,
            audiobooks: value.audiobooks,
            active_audiobooks: value.active_audiobooks,
            finished_audiobooks: value.finished_audiobooks,
        }
    }
}
