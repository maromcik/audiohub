use askama::Template;
use crate::database::models::chapter::Chapter;

#[derive(Template)]
#[template(path = "chapter/chapter_create.html")]
pub struct ChapterCreateFormTemplate {}

#[derive(Template)]
#[template(path = "chapter/chapters_all.html")]
pub struct ChaptersAllTemplate {
    pub chapters: Vec<Chapter>
}