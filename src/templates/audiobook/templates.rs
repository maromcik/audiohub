use askama::Template;

#[derive(Template)]
#[template(path = "new_audiobook.html")]
pub struct NewAudiobookForm {}
