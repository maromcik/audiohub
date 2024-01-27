use crate::database::models::active_audiobook::PlayedAudiobook;
use crate::database::models::audiobook::{Audiobook, AudiobookDisplay, AudiobookQuickSearch, AudiobookRecommenderCard, AudiobookRecommenderDisplay};
use crate::database::models::chapter::ChapterDisplay;
use crate::database::models::genre::Genre;
use askama::Template;


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
#[template(path = "audiobook/book_cover_thumbnail.html")]
pub struct AudiobookThumbnail {
    pub message: String,
    pub audiobook: AudiobookDisplay,
}

#[derive(Template)]
#[template(path = "audiobook/book_cover_update.html")]
pub struct AudiobookCoverUpload {
    pub message: String,
    pub audiobook: AudiobookDisplay,
}

#[derive(Template)]
#[template(path = "audiobook/recommendations.html")]
pub struct AudiobookRecommendationTemplate {
    pub books: Vec<AudiobookRecommenderDisplay>
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
#[template(path = "components/detail_likes.html")]
pub struct DetailLikesTemplate {
    pub likes: i64,
    pub is_liked: bool,
}

#[derive(Template)]
#[template(path = "components/card_likes.html")]
pub struct CardLikesTemplate {
    pub likes: i64,
    pub is_liked: bool,
}

#[derive(Template)]
#[template(path = "audiobook/audiobooks_by_genre.html")]
pub struct AudiobooksByGenreTemplate {
    pub audiobooks: Vec<AudiobookDisplay>,
    pub genre: Genre,
}

#[derive(Template)]
#[template(path = "audiobook/audiobooks_by_genre_content.html")]
pub struct AudiobooksByGenreContentTemplate {
    pub audiobooks: Vec<AudiobookDisplay>,
    pub genre: Genre,
}

pub struct AudiobooksByGenreBase {
    pub audiobooks: Vec<AudiobookDisplay>,
    pub genre: Genre,
}

impl From<AudiobooksByGenreBase> for AudiobooksByGenreTemplate {
    fn from(value: AudiobooksByGenreBase) -> Self {
        Self {
            audiobooks: value.audiobooks,
            genre: value.genre
        }
    }
}

impl From<AudiobooksByGenreBase> for AudiobooksByGenreContentTemplate {
    fn from(value: AudiobooksByGenreBase) -> Self {
        Self {
            audiobooks: value.audiobooks,
            genre: value.genre
        }
    }
}


#[derive(Template)]
#[template(path = "detail.html")]
pub struct AudiobookDetailPageTemplate {
    pub audiobook: AudiobookDisplay,
    pub chapters: Vec<ChapterDisplay>,
    pub is_liked: bool,
}

#[derive(Template)]
#[template(path = "audiobook/detail-content.html")]
pub struct AudiobookDetailContentTemplate {
    pub audiobook: AudiobookDisplay,
    pub chapters: Vec<ChapterDisplay>,
    pub is_liked: bool,
}

#[derive(Template)]
#[template(path = "detail_author.html")]
pub struct AudiobookDetailAuthorPageTemplate {
    pub audiobook: AudiobookDisplay,
    pub chapters: Vec<ChapterDisplay>,
    pub is_liked: bool,
}

#[derive(Template)]
#[template(path = "audiobook/detail_author-content.html")]
pub struct AudiobookDetailAuthorContentTemplate {
    pub audiobook: AudiobookDisplay,
    pub chapters: Vec<ChapterDisplay>,
    pub is_liked: bool,
}

#[derive(Template)]
#[template(path = "components/player.html")]
pub struct PlayerTemplate {
    pub played_book: PlayedAudiobook,
}

#[derive(Template)]
#[template(path = "components/search-results.html")]
pub struct QuickSearchResults {
    pub results: Vec<AudiobookQuickSearch>,
}

pub struct AudiobookDetailBase {
    pub audiobook: AudiobookDisplay,
    pub chapters: Vec<ChapterDisplay>,
    pub is_liked: bool
}

impl From<AudiobookDetailBase> for AudiobookDetailPageTemplate {
    fn from(value: AudiobookDetailBase) -> Self {
        Self {
            audiobook: value.audiobook,
            chapters: value.chapters,
            is_liked: value.is_liked,
        }
    }
}

impl From<AudiobookDetailBase> for AudiobookDetailContentTemplate {
    fn from(value: AudiobookDetailBase) -> Self {
        Self {
            audiobook: value.audiobook,
            chapters: value.chapters,
            is_liked: value.is_liked,
        }
    }
}

#[derive(Template)]
#[template(path = "studio_edit_audiobook.html")]
pub struct AudiobookEditPageTemplate {
    pub genres: Vec<Genre>,
    pub audiobook: AudiobookDisplay,
}
#[derive(Template)]
#[template(path = "audiobook/audiobook_edit.html")]
pub struct AudiobookEditContentTemplate {
    pub genres: Vec<Genre>,
    pub audiobook: AudiobookDisplay,
}

pub struct AudiobookEditBase {
    pub genres: Vec<Genre>,
    pub audiobook: AudiobookDisplay,
}

impl From<AudiobookEditBase> for AudiobookEditContentTemplate {
    fn from(value: AudiobookEditBase) -> Self {
        Self {
            genres: value.genres,
            audiobook: value.audiobook,
        }
    }
}

impl From<AudiobookEditBase> for AudiobookEditPageTemplate {
    fn from(value: AudiobookEditBase) -> Self {
        Self {
            genres: value.genres,
            audiobook: value.audiobook,
        }
    }
}