use askama::Template;
use crate::database::models::audiobook::Audiobook;

#[derive(Template)]
#[template(path = "audiobook/audiobook_create.html")]
pub struct AudiobookCreateFormTemplate {}

#[derive(Template)]
#[template(path = "audiobook/audiobook_upload.html")]
pub struct AudiobookUploadFormTemplate {}

#[derive(Template)]
#[template(path = "audiobook/audiobook_detail_owner.html")]
pub struct AudiobookDetailOwnerTemplate {
    pub audiobook: Audiobook
}
