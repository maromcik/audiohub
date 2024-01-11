// handlers.rs
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::templates::user::{LoginTemplate, RegistrationTemplate};
use actix_web::{get, post, web, HttpResponse};
use askama::Template;

pub use hmac;

use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Pbkdf2
};
use crate::database::models::user::UserCreate;

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

#[post("/register")]
pub async fn create_user(form: web::Form<NewUserForm>,
                         mut user_repo: web::Data<UserRepository>) -> Result<HttpResponse, AppError>{

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

fn hash_password(password: String, salt: SaltString) -> Result<String, pbkdf2::password_hash::Error >{
    let password_hash = Pbkdf2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}