use crate::database::models::audiobook::{AudiobookDetail, AudiobookDisplay};
use crate::database::models::chapter::ChapterDisplay;
use crate::database::models::genre::Genre;
use askama::Template;
use crate::database::models::active_audiobook::PlayedAudiobook;

#[derive(Template)]
#[template(path = "studio_create_audiobook.html")]
pub struct AudiobookCreatePageTemplate {
    pub genres: Vec<Genre>,
}

#[derive(Template)]
#[template(path = "audiobook/audiobook_create.html")]
pub struct AudiobookCreateContentTemplate {
    pub genres: Vec<Genre>,
}

#[derive(Template)]
#[template(path = "audiobook/audiobook_upload.html")]
pub struct AudiobookUploadFormTemplate {
    pub message: String,
}

#[derive(Template)]
#[template(path = "releases.html")]
pub struct NewReleasesPageTemplate {
    pub audiobooks: Vec<AudiobookDisplay>,
}

#[derive(Template)]
#[template(path = "audiobook/releases-content.html")]
pub struct NewReleasesContentTemplate {
    pub audiobooks: Vec<AudiobookDisplay>,
}

#[derive(Template)]
#[template(path = "audiobook/audiobooks_by_genre.html")]
pub struct AudiobooksByGenreTemplate {
    pub audiobooks: Vec<AudiobookDisplay>,
    pub genre_name: String,
}

#[derive(Template)]
#[template(path = "audiobook/audiobook_detail.html")]
pub struct AudiobookDetailPageTemplate {
    pub audiobook: AudiobookDisplay,
    pub chapters: Vec<ChapterDisplay>,
}

#[derive(Template)]
#[template(path = "audiobook/audiobook_detail.html")]
pub struct AudiobookDetailContentTemplate {
    pub audiobook: AudiobookDisplay,
    pub chapters: Vec<ChapterDisplay>,
}

#[derive(Template)]
#[template(path = "components/player.html")]
pub struct PlayerTemplate {
    pub played_book: PlayedAudiobook,
}



pub struct AudiobookDetailBase {
    pub audiobook: AudiobookDisplay,
    pub chapters: Vec<ChapterDisplay>,
}

impl From<AudiobookDetailPageTemplate> for AudiobookDetailBase {
    fn from(value: AudiobookDetailPageTemplate) -> Self {
        Self {
            audiobook: value.audiobook,
            chapters: value.chapters
        }
    }
}

impl From<AudiobookDetailContentTemplate> for AudiobookDetailBase {
    fn from(value: AudiobookDetailContentTemplate) -> Self {
        Self {
            audiobook: value.audiobook,
            chapters: value.chapters
        }
    }
}