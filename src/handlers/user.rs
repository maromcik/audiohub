use crate::database::common::error::{BusinessLogicErrorKind, DbError, DbResultSingle};
use crate::database::common::DbCreate;
use crate::database::models::user::UserCreate;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use actix_web::error::ErrorInternalServerError;
use actix_web::{get, web, HttpResponse};

#[get("/index")]
pub async fn index(user_repo: web::Data<UserRepository>) -> Result<HttpResponse, AppError> {
    // let template = crate::templates::user::RegistrationTemplate {};
    // let body = template.render().map_err(ErrorInternalServerError)?;
    // db is broken, I dont have time rn.
    // let b = user_repo.create(&UserCreate::new("", "", "", "", "", "", "", "")).await?;
    Ok(HttpResponse::Ok().finish())
}
