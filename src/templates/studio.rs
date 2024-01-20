use askama::Template;
use crate::database::models::audiobook::AudiobookDetail;
use crate::database::models::chapter::ChapterDisplay;

#[derive(Template)]
#[template(path = "studio.html")]
pub struct StudioPageTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
}

#[derive(Template)]
#[template(path = "audiobook/studio-content.html")]
pub struct StudioContentTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
}