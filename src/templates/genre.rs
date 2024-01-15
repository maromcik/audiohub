use askama::Template;
use crate::database::models::genre::Genre;

#[derive(Template)]
#[template(path = "genre/genres.html")]
pub struct AllGenresTemplate {
    pub genres: Vec<Genre>,
}