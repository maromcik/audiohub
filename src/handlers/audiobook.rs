use crate::database::common::{DbCreate, DbReadMany, DbReadOne};
use crate::database::models::audiobook::{AudiobookCreate, AudiobookCreateForm};
use crate::database::models::user::UserGetByUsername;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::{AppError, AppErrorKind};
use crate::templates::audiobook::NewAudiobookForm;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use sqlx::postgres::types::PgInterval;
use crate::database::models::genre::{GenreGetById, GenreSearch};
use crate::database::repositories::genre::repository::GenreRepository;

#[get("/create")]
pub async fn create_audiobook_form() -> Result<HttpResponse, AppError> {
    let template = NewAudiobookForm {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/create")]
pub async fn create_audiobook(
    identity: Option<Identity>,
    audiobook_repo: web::Data<AudiobookRepository>,
    genre_repo: web::Data<GenreRepository>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<AudiobookCreateForm>,
) -> Result<HttpResponse, AppError> {
    let Some(u) = identity else {
        return Err(AppError::new(
            AppErrorKind::IdentityError,
            "Invalid identity",
        ));
    };
    let user = user_repo
        .read_one(&UserGetByUsername::new(&u.id()?))
        .await?;
    let genre_id = match genre_repo.read_many(&GenreSearch::new(&form.genre_name)).await?.first() {
        Some(g) => g.id,
        None => 1
    };
    let book = AudiobookCreate::new(
        &form.name,
        &user.id,
        &genre_id,
        &20,
        &0,
        &PgInterval {
            months: 0,
            days: 0,
            microseconds: 100,
        },
        "",
        &0,
        &0,
        "",
        "",
    );
    audiobook_repo.create(&book).await?;
    Ok(HttpResponse::Ok().finish())
}
