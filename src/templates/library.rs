use askama::Template;
use crate::database::models::audiobook::AudiobookDetail;

#[derive(Template)]
#[template(path = "library.html")]
pub struct LibraryPageTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
    // TODO VOSY: change to ActiveBook
    pub audiobook: AudiobookDetail
}

#[derive(Template)]
#[template(path = "library-content.html")]
pub struct LibraryContentTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
    // TODO VOSY: change to ActiveBook
    pub audiobook: AudiobookDetail
}