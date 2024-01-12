use actix_identity::Identity;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::error::{AppError, AppErrorKind};
use crate::templates::audiobook::NewAudiobookForm;
use actix_web::{get, web, HttpResponse, post};
use askama::Template;
use sqlx::postgres::types::PgInterval;
use crate::database::common::{DbCreate, DbReadOne};
use crate::database::models::audiobook::{AudiobookCreate, AudiobookCreateForm};
use crate::database::models::user::UserGetByUsername;
use crate::database::repositories::user::repository::UserRepository;

#[get("/audiobook/create")]
pub async fn create_audiobook_form() -> Result<HttpResponse, AppError> {
    let template = NewAudiobookForm {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/audiobook/create")]
pub async fn create_audiobook(identity: Option<Identity>, audiobook_repo: web::Data<AudiobookRepository>, user_repo: web::Data<UserRepository>, form: web::Form<AudiobookCreateForm>) -> Result<HttpResponse, AppError> {
    let Some(u) = identity else {
        return Err(AppError::new(AppErrorKind::IdentityError, "Invalid identity"));
    };
    let user = user_repo.read_one(&UserGetByUsername::new(&u.id()?)).await?;
    let book = AudiobookCreate::new(
        &form.name,
        &user.id,
        &1,
        &20,
        &0, &PgInterval { months: 0, days: 0, microseconds: 100 },
        "",
        &0,
        &0,
        "",
        "");
    audiobook_repo.create(&book).await?;
    Ok(HttpResponse::Ok().finish())
}