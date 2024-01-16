use crate::database::models::chapter::Chapter;
use askama::Template;
use crate::database::models::Id;

#[derive(Template)]
#[template(path = "chapter/chapter_create.html")]
pub struct ChapterCreateFormTemplate {
    pub audiobook_id: Id,
    pub position: f64,
}

#[derive(Template)]
#[template(path = "chapter/chapters_by_audiobook.html")]
pub struct ChaptersByAudiobookTemplate {
    pub chapters: Vec<Chapter>,
}

#[derive(Template)]
#[template(path = "chapter/chapter_detail.html")]
pub struct ChapterDetailTemplate {
    pub chapter: Chapter
}