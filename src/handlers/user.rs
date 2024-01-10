// handlers.rs
use crate::database::repositories::user::repository::UserRepository;
use crate::templates::{LoginTemplate, RegistrationTemplate};

use actix_web::{
    delete, error::ErrorInternalServerError, get, patch, post, put, web, HttpResponse,
    Result as ActixResult,
};
use askama::Template;

#[get("/register")]
pub async fn register(user_repo: web::Data<UserRepository>) -> ActixResult<HttpResponse> {
    let template = RegistrationTemplate {};
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/login")]
pub async fn login(user_repo: web::Data<UserRepository>) -> ActixResult<HttpResponse> {
    let template = LoginTemplate {};
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
