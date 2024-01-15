use crate::database::models::audiobook::AudiobookDetail;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub logged_in: bool,
    pub username: String,
    pub audiobooks: Vec<AudiobookDetail>,
}
