use askama::Template;

#[derive(Template)]
#[template(path = "audiobook/audiobook_new.html")]
pub struct NewAudiobookForm {}
