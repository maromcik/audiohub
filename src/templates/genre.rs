use crate::database::models::genre::Genre;
use askama::Template;

#[derive(Template)]
#[template(path = "genres.html")]
pub struct GenresPageTemplate {
    pub genres: Vec<Genre>,
}

#[derive(Template)]
#[template(path = "genre/genres-content.html")]
pub struct GenresContentTemplate {
    pub genres: Vec<Genre>,
}
