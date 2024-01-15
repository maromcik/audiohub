use crate::database::models::audiobook::{Audiobook, AudiobookDetail};
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
#[template(path = "audiobook/audiobook_detail_owner.html")]
pub struct AudiobookDetailOwnerTemplate {
    pub audiobook: Audiobook,
}

#[derive(Template)]
#[template(path = "audiobook/releases.html")]
pub struct NewReleasesTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
}

#[derive(Template)]
#[template(path = "audiobook/audiobooks_by_genre.html")]
pub struct AudiobooksByGenreTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
    pub genre_name: String,
}