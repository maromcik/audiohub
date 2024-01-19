use crate::database::models::audiobook::AudiobookDetail;
use crate::database::models::chapter::ChapterDisplay;
use crate::database::models::genre::Genre;
use askama::Template;
use crate::database::models::active_audiobook::PlayedAudiobook;

#[derive(Template)]
#[template(path = "studio.html")]
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
pub struct AudiobookUploadFormTemplate {}

#[derive(Template)]
#[template(path = "releases.html")]
pub struct NewReleasesPageTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
}

#[derive(Template)]
#[template(path = "audiobook/releases-content.html")]
pub struct NewReleasesContentTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
}

#[derive(Template)]
#[template(path = "audiobook/audiobooks_by_genre.html")]
pub struct AudiobooksByGenreTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
    pub genre_name: String,
}

#[derive(Template)]
#[template(path = "detail.html")]
pub struct AudiobookDetailPageTemplate {
    pub audiobook: AudiobookDetail,
    pub chapters: Vec<ChapterDisplay>,
}

#[derive(Template)]
#[template(path = "audiobook/detail-content.html")]
pub struct AudiobookDetailContentTemplate {
    pub audiobook: AudiobookDetail,
    pub chapters: Vec<ChapterDisplay>,
}

#[derive(Template)]
#[template(path = "components/player.html")]
pub struct PlayerTemplate {
    pub last_played: PlayedAudiobook,
}



pub struct AudiobookDetailBase {
    pub audiobook: AudiobookDetail,
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