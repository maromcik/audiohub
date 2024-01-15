use crate::database::models::chapter::Chapter;
use askama::Template;

#[derive(Template)]
#[template(path = "chapter/chapter_create.html")]
pub struct ChapterCreateFormTemplate {}

#[derive(Template)]
#[template(path = "chapter/chapters_by_audiobook.html")]
pub struct ChaptersByAudiobookTemplate {
    pub chapters: Vec<Chapter>,
}
