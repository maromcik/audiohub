use crate::database::models::chapter::ChapterDisplay;
use crate::database::models::Id;
use askama::Template;

#[derive(Template)]
#[template(path = "components/chapter-create-player.html")]
pub struct ChapterCreatorPlayerTemplate {
    pub source: String,
}

#[derive(Template)]
#[template(path = "chapter/chapter-timeline.html")]
pub struct ChapterTimelineTemplate {
    pub audiobook_id: Id,
    pub chapters: Vec<ChapterDisplay>,
    pub length: f64,
}

#[derive(Template)]
#[template(path = "chapter/chapter-list.html")]
pub struct ChapterListTemplate {
    pub audiobook_id: Id,
    pub chapters: Vec<ChapterDisplay>,
    pub show_delete: bool,
}
