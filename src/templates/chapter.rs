use crate::database::models::chapter::Chapter;
use crate::database::models::Id;
use askama::Template;

#[derive(Template)]
#[template(path = "chapter/chapter_create.html")]
pub struct ChapterCreateFormTemplate {
    pub audiobook_id: Id,
}

#[derive(Template)]
#[template(path = "chapter/chapter_detail.html")]
pub struct ChapterDetailTemplate {
    pub chapter: Chapter,
}
