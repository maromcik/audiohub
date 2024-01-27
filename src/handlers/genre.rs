use crate::authorized;
use crate::database::common::{DbReadMany};

use crate::database::models::genre::{GenreSearch};
use crate::database::models::Id;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::genre::repository::GenreRepository;
use crate::error::AppError;

use crate::templates::audiobook::{AudiobooksByGenreTemplate, AudiobooksByGenreContentTemplate};
use crate::templates::genre::{GenresContentTemplate, GenresPageTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpResponse, HttpRequest};
use askama::Template;
use crate::handlers::helpers::get_genre_base;

#[get("/all")]
async fn get_genres_page(
    request: HttpRequest,
    identity: Option<Identity>,
    genre_repo: web::Data<GenreRepository>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    //get all genres
    let genres = genre_repo.read_many(&GenreSearch::default()).await?;

    let template = GenresPageTemplate { genres };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/content")]
async fn get_genres_content(
    request: HttpRequest,
    identity: Option<Identity>,
    genre_repo: web::Data<GenreRepository>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    // get all genres
    let genres = genre_repo.read_many(&GenreSearch::default()).await?;

    let template = GenresContentTemplate { genres };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}")]
async fn get_audiobooks_by_genre(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    genre_repo: web::Data<GenreRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let template = AudiobooksByGenreTemplate::from(get_genre_base(
        u,
        audiobook_repo,
        genre_repo,
        path.into_inner().0
    ).await?);
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}/content")]
async fn get_audiobooks_by_genre_content(
    request: HttpRequest,
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    genre_repo: web::Data<GenreRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let template = AudiobooksByGenreContentTemplate::from(get_genre_base(
        u,
        audiobook_repo,
        genre_repo,
        path.into_inner().0
    ).await?);
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}