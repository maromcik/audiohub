use crate::database::models::audiobook::{AudiobookDetail, AudiobookDisplay};
use askama::Template;

#[derive(Template)]
#[template(path = "library.html")]
pub struct LibraryPageTemplate {
    pub audiobooks: Vec<AudiobookDisplay>,
}

#[derive(Template)]
#[template(path = "audiobook/library-content.html")]
pub struct LibraryContentTemplate {
    pub audiobooks: Vec<AudiobookDisplay>,
}
