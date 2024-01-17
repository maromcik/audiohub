use askama::Template;
use crate::database::models::audiobook::AudiobookDetail;

#[derive(Template)]
#[template(path = "library.html")]
pub struct LibraryPageTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
}