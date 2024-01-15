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
#[template(path = "audiobook/new_releases.html")]
pub struct NewReleasesTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
}
