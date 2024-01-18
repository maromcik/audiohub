use crate::database::models::audiobook::{AudiobookDetail};
use crate::database::models::chapter::{Chapter, ChapterDisplay};
use crate::database::models::genre::Genre;
use askama::Template;

#[derive(Template)]
#[template(path = "audiobook/audiobook_create.html")]
pub struct AudiobookCreateFormTemplate {
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
#[template(path = "audiobook/audiobook_detail.html")]
pub struct AudiobookDetailPageTemplate {
    pub audiobook: AudiobookDetail,
    pub chapters: Vec<ChapterDisplay>,
}
