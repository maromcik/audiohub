use askama::Template;
use crate::database::models::audiobook::{AudiobookDisplay};
use crate::database::models::chapter::ChapterDisplay;

#[derive(Template)]
#[template(path = "studio.html")]
pub struct StudioPageTemplate {
    pub audiobooks: Vec<AudiobookDisplay>,
}

#[derive(Template)]
#[template(path = "audiobook/studio-content.html")]
pub struct StudioContentTemplate {
    pub audiobooks: Vec<AudiobookDisplay>,
}