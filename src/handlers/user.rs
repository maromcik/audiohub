use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::templates::homepage_template::HomepageTemplate;
use crate::templates::user::{LoginTemplate, RegistrationTemplate};
use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use actix_web::http::header::LOCATION;

pub use hmac;

use crate::database::common::{DbCreate, DbReadOne};
use crate::database::models::user::{LoginUser, NewUserForm, UserCreate, UserLogin};
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use uuid::Uuid;

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


#[post("/register")]
pub async fn register_user(
    form: web::Form<NewUserForm>,
    mut user_repo: web::Data<UserRepository>,
) -> Result<HttpResponse, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password =
        hash_password(form.password.to_string(), salt).unwrap_or_else(|_| "".to_string());

    let new_user = UserCreate {
        username: form.username.to_string(),
        email: form.email.to_string(),
        name: form.name.to_string(),
        surname: form.surname.to_string(),
        bio: "".to_string(),
        profile_picture: "".to_string(),
        password_hash: hashed_password.clone(),
        password_salt: "".to_string(),
    };

    user_repo.create(&new_user).await?;
    user_repo
        .read_one(&UserLogin::new(&form.email, &hashed_password.clone()))
        .await?;

    let template = HomepageTemplate {};
    let body = template.render()?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/user/login"))
        .finish())
}


#[post("/login")]
pub async fn login_user(user_repo: web::Data<UserRepository>, form: web::Form<LoginUser>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish())
}

fn hash_password(
    password: String,
    salt: SaltString,
) -> Result<String, pbkdf2::password_hash::Error> {
    let password_hash = Pbkdf2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

async fn validate_credentials() -> Result<Uuid, AppError> {
    todo!()
}

fn verify_password_hash(expected_password_hash: &str, password_candidate: String) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(expected_password_hash)?;
    Ok(Pbkdf2.verify_password(&password_candidate.into_bytes(), &parsed_hash).is_ok())
}