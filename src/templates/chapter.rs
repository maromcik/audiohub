use askama::Template;

#[derive(Template)]
#[template(path = "chapter/chapter_create.html")]
pub struct ChapterCreateFormTemplate {}