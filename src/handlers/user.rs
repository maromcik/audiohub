use actix_web::{get, HttpResponse, web};
use actix_web::error::ErrorInternalServerError;
use crate::database::common::DbCreate;
use crate::database::models::user::UserCreate;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;

#[get("/index")]
pub async fn index(mut user_repo: web::Data<UserRepository>) -> Result<HttpResponse, AppError> {
    // let template = crate::templates::user::RegistrationTemplate {};
    // let body = template.render().map_err(ErrorInternalServerError)?;
    let b = user_repo.create(&UserCreate::new("", "", "", "", "", "", "", "")).await?;
    Ok(HttpResponse::Ok().finish())
}