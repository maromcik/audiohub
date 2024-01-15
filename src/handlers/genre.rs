use actix_web::{get, HttpResponse, web};
use crate::database::common::{DbReadMany, DbReadOne};
use crate::database::models::genre::{GenreGetById, GenreSearch};
use crate::database::repositories::genre::repository::GenreRepository;
use crate::error::AppError;
use crate::templates::genre::AllGenresTemplate;
use askama::Template;
use crate::database::models::audiobook::AudiobookSearch;
use crate::database::models::Id;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::templates::audiobook::AudiobooksByGenreTemplate;

#[get("/")]
async fn get_genres(genre_repo: web::Data<GenreRepository>) -> Result<HttpResponse, AppError> {
    //get all genres
    let genres = genre_repo.read_many(&GenreSearch::new(None)).await?;

    let template = AllGenresTemplate { genres };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}")]
async fn get_audiobooks_by_genre(
    audiobook_repo: web::Data<AudiobookRepository>,
    genre_repo: web::Data<GenreRepository>,
    path: web::Path<(Id,)>) -> Result<HttpResponse, AppError> {
    let genre_id = path.into_inner().0;
    let book_search = AudiobookSearch::search_by_genre_id(Some(genre_id));
    let books = audiobook_repo
        .read_many(&book_search)
        .await?;

    let genre = genre_repo
        .read_one(&GenreGetById::new(&genre_id))
        .await?;

    let template = AudiobooksByGenreTemplate { audiobooks: books, genre_name: genre.name};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}