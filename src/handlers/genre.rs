use actix_web::{get, HttpResponse, web};
use crate::database::common::DbReadMany;
use crate::database::models::genre::GenreSearch;
use crate::database::repositories::genre::repository::GenreRepository;
use crate::error::AppError;
use crate::templates::genre::AllGenresTemplate;
use askama::Template;

#[get("/")]
async fn get_genres(genre_repo: web::Data<GenreRepository>) -> Result<HttpResponse, AppError> {
    //get all genres
    let genres = genre_repo.read_many(&GenreSearch::new(None)).await?;

    let template = AllGenresTemplate { genres: genres };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}