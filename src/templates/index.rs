use askama::Template;
use crate::database::models::audiobook::Audiobook;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub logged_in: bool,
    pub username: String,
    pub audiobooks: Vec<Audiobook>,
}
