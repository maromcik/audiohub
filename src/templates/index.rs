use askama::Template;
use crate::database::models::audiobook::{Audiobook, AudiobookDetail};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub logged_in: bool,
    pub username: String,
    pub audiobooks: Vec<AudiobookDetail>,
}
