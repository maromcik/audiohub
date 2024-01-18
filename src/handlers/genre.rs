use crate::authorized;
use crate::database::common::{DbReadMany, DbReadOne};
use crate::database::models::audiobook::AudiobookSearch;
use crate::database::models::genre::{GenreGetById, GenreSearch};
use crate::database::models::Id;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::genre::repository::GenreRepository;
use crate::error::AppError;
use crate::templates::audiobook::AudiobooksByGenreTemplate;
use crate::templates::genre::{GenresContentTemplate, GenresPageTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse};
use askama::Template;

#[get("/all")]
async fn get_genres_page(
    identity: Option<Identity>,
    genre_repo: web::Data<GenreRepository>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    //get all genres
    let genres = genre_repo.read_many(&GenreSearch::default()).await?;

    let template = GenresPageTemplate { genres };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/content")]
async fn get_genres_content(
    identity: Option<Identity>,
    genre_repo: web::Data<GenreRepository>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    // get all genres
    let genres = genre_repo.read_many(&GenreSearch::default()).await?;

    let template = GenresContentTemplate { genres };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}


#[get("/{id}")]
async fn get_audiobooks_by_genre(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    genre_repo: web::Data<GenreRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let genre_id = path.into_inner().0;
    let book_search = AudiobookSearch::search_by_genre_id(genre_id);
    let books = audiobook_repo.read_many(&book_search).await?;

    let genre = genre_repo.read_one(&GenreGetById::new(&genre_id)).await?;

    let template = AudiobooksByGenreTemplate {
        audiobooks: books,
        genre_name: genre.name,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
