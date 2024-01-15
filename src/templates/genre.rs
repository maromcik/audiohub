use crate::database::models::genre::Genre;
use askama::Template;

#[derive(Template)]
#[template(path = "genre/genres.html")]
pub struct AllGenresTemplate {
    pub genres: Vec<Genre>,
}
