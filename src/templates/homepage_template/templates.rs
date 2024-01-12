use askama::Template;

#[derive(Template)]
#[template(path = "homepage.html")]
pub struct HomepageTemplate {}
