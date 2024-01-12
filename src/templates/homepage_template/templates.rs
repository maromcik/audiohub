use crate::database::models::Id;
use askama::Template;

#[derive(Template)]
#[template(path = "homepage.html")]
pub struct HomepageTemplate {
    pub logged_in: bool,
    pub username: String,
}
