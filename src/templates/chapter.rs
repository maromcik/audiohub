use crate::database::models::chapter::{Chapter, ChapterDisplay};
use askama::Template;
use crate::database::models::Id;


#[derive(Template)]
#[template(path = "chapter/chapter_detail.html")]
pub struct ChapterDetailTemplate {
    pub chapter: Chapter,
}

#[derive(Template)]
#[template(path = "components/chapter-create-player.html")]
pub struct ChapterCreatorPlayerTemplate {
    pub source: String,
}

#[derive(Template)]
#[template(path = "chapter/chapter-timeline.html")]
pub struct ChapterTimelineTemplate {
    pub book_id: Id,
    pub chapters: Vec<ChapterDisplay>,
    pub length: f64,
}

#[derive(Template)]
#[template(path = "chapter/chapter-list.html")]
pub struct ChapterListTemplate {
    pub book_id: Id,
    pub chapters: Vec<ChapterDisplay>,
}
