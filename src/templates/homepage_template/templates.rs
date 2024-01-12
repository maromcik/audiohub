use askama::Template;
use crate::database::models::Id;

#[derive(Template)]
#[template(path = "homepage.html")]
pub struct HomepageTemplate {
    pub logged_in: bool,
    pub username: String
}
