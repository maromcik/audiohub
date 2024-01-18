use crate::database::models::audiobook::AudiobookDetail;
use askama::Template;

#[derive(Template)]
#[template(path = "pages/library.html")]
pub struct LibraryPageTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
}

#[derive(Template)]
#[template(path = "audiobook/library-content.html")]
pub struct LibraryContentTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
}
