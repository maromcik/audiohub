// handlers.rs
use actix_files::{Files as ActixFiles, Files};
use actix_web::{
    delete, error::ErrorInternalServerError, get, patch, post, put, web, App, HttpResponse,
    HttpServer, Result as ActixResult,
};
use askama::Template;
use sqlx::PgPool;
use crate::database::common::DbReadOne;
use crate::database::models::user::{UserCreate, UserLogin};
use crate::database::repositories::user::repository::UserRepository;
use crate::templates::{LoginTemplate, RegistrationTemplate}; // Assume UserForm is a struct representing the form input


pub async fn register(user_repo: web::Data<UserRepository>) -> ActixResult<HttpResponse> {
    let template = RegistrationTemplate{ };
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub async fn login(user_repo: web::Data<UserRepository>) -> ActixResult<HttpResponse> {
    let template = LoginTemplate{ };
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}