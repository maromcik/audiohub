// handlers.rs
use actix_files::{Files as ActixFiles, Files};
use actix_web::{delete, error::ErrorInternalServerError, get, patch, post, put, web, App, HttpResponse, HttpServer, Result as ActixResult, ResponseError};
pub use hmac;

use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Pbkdf2
};
use actix_web::http::StatusCode;

use askama::Template;
use pbkdf2::password_hash::Error;
use sqlx::PgPool;
use crate::database::common::{DbCreate, DbReadOne};
use crate::database::models::user::{UserCreate, UserLogin};
use crate::database::repositories::user::repository::UserRepository;
use crate::templates::{LoginTemplate, RegistrationTemplate}; // Assume UserForm is a struct representing the form input

#[get("/register")]
pub async fn register(user_repo: web::Data<UserRepository>) -> ActixResult<HttpResponse> {
    let template = RegistrationTemplate{ };
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[derive(serde::Deserialize)]
pub struct NewUserForm {
    username: String,
    email: String,
    password: String,
    name: String,
    surname: String,
    bio: String,
    profile_picture: String,
}

#[derive(serde::Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String,
}

#[post("/register")]
pub async fn create_user(form: web::Form<NewUserForm>,
                         mut user_repo: web::Data<UserRepository>) -> ActixResult<HttpResponse>{

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = hash_password(form.password.to_string(), salt).unwrap_or_else(|_| "".to_string());

    let new_user = UserCreate{
        username: form.username.to_string(),
        email: form.email.to_string(),
        name: form.name.to_string(),
        surname: form.surname.to_string(),
        bio: form.bio.to_string(),
        profile_picture: form.profile_picture.to_string(),
        password_hash: hashed_password,
        password_salt: "".to_string(),
    };

    //let created_user = user_repo.create(&new_user).await?;

    Ok(HttpResponse::Ok().content_type("text/html").body(new_user.username))

}

#[derive(serde::Deserialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

pub async fn login_user(form: web::Form<LoginUser>,
                        mut user_repo: web::Data<UserRepository>) -> ActixResult<HttpResponse>{
    let user_login = UserLogin{ email: form.email.to_string(), password_hash: form.password.to_string() };
    //let db_user = user_repo.read_one(&user_login).await?;

    //verify passwords
    //login

    Ok(HttpResponse::Ok().content_type("text/html").body(user_login.email))
}

pub async fn login(user_repo: web::Data<UserRepository>) -> ActixResult<HttpResponse> {
    let template = LoginTemplate{ };
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

fn hash_password(password: String, salt: SaltString) -> Result<String, pbkdf2::password_hash::Error >{
    let password_hash = Pbkdf2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}