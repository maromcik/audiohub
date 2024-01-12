use actix_identity::Identity;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::templates::homepage_template::HomepageTemplate;
use crate::templates::user::{LoginTemplate, RegistrationTemplate};
use actix_web::{get, post, web, HttpResponse, HttpRequest, HttpMessage};
use askama::Template;
use actix_web::http::header::LOCATION;


use crate::database::common::{DbCreate, DbReadOne};
use crate::database::models::user::{NewUserForm, UserCreate, UserLogin};

use uuid::Uuid;

#[get("/register")]
pub async fn register() -> Result<HttpResponse, AppError> {
    let template = RegistrationTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/login")]
pub async fn login() -> Result<HttpResponse, AppError> {
    let template = LoginTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}


#[post("/register")]
pub async fn register_user(
    form: web::Form<NewUserForm>,
    user_repo: web::Data<UserRepository>,
) -> Result<HttpResponse, AppError> {
    let new_user = UserCreate {
        username: form.username.to_string(),
        email: form.email.to_string(),
        name: form.name.to_string(),
        surname: form.surname.to_string(),
        bio: "".to_string(),
        profile_picture: "".to_string(),
        password: form.password.clone()
    };

    user_repo.create(&new_user).await?;
    user_repo
        .read_one(&UserLogin::new(&form.email, &form.password))
        .await?;

    let template = HomepageTemplate {};
    let body = template.render()?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/user/login"))
        .finish())
}


#[post("/login")]
pub async fn login_user(request: HttpRequest, user_repo: web::Data<UserRepository>, form: web::Form<UserLogin>) -> Result<HttpResponse, AppError> {
    // let login = user_repo.read_one(&UserLogin::new("", ))
    // let b = Identity::login(&request.extensions(), "User1".into());
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish())
}
