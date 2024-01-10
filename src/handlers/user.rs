// handlers.rs
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::templates::user::{LoginTemplate, RegistrationTemplate};
use actix_web::{get, web, HttpResponse};
use askama::Template;

#[get("/register")]
pub async fn register(user_repo: web::Data<UserRepository>) -> Result<HttpResponse, AppError> {
    let template = RegistrationTemplate {};
    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/login")]
pub async fn login(user_repo: web::Data<UserRepository>) -> Result<HttpResponse, AppError> {
    let template = LoginTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
