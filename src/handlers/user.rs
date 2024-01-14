use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::templates::user::{LoginTemplate, RegistrationTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use askama::Template;

use crate::database::common::{DbCreate, DbReadOne};
use crate::database::models::user::{UserCreate, UserLogin};
use crate::forms::user::UserCreateForm;

#[get("/register")]
pub async fn register() -> Result<HttpResponse, AppError> {
    let template = RegistrationTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/login")]
pub async fn login() -> Result<HttpResponse, AppError> {
    let template = LoginTemplate { message: "".to_string()};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/register")]
pub async fn register_user(
    form: web::Form<UserCreateForm>,
    user_repo: web::Data<UserRepository>,
) -> Result<HttpResponse, AppError> {
    let new_user = UserCreate {
        username: form.username.to_string(),
        email: form.email.to_string(),
        name: form.name.to_string(),
        surname: form.surname.to_string(),
        bio: String::new(),
        profile_picture: String::new(),
        password: form.password.clone(),
    };

    user_repo.create(&new_user).await?;
    user_repo
        .read_one(&UserLogin::new(&form.email, &form.password))
        .await?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/user/login"))
        .finish())
}

#[post("/login")]
pub async fn login_user(
    request: HttpRequest,
    user_repo: web::Data<UserRepository>,
    form: web::Form<UserLogin>,
) -> Result<HttpResponse, AppError> {
    match user_repo
        .read_one(&UserLogin::new(&form.email_or_username, &form.password))
        .await
    {
        Ok(user) => {
            Identity::login(&request.extensions(), user.username.clone())?;
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .finish())
        },
        Err(_) => {
            let template = LoginTemplate { message: "".to_string()};
            let body = template.render()?;
            Ok(HttpResponse::Ok().content_type("text/html").body(body))
        }
    }
}
