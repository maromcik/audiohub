use crate::database::common::{DbDelete, DbReadOne};
use crate::database::models::user::{UserDelete, UserLogin};
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::templates::user::Index;
use actix_web::{get, web, HttpResponse};
use askama::Template;

#[get("/index")]
pub async fn index(user_repo: web::Data<UserRepository>) -> Result<HttpResponse, AppError> {
    let template = Index {};
    let body = template.render()?;
    let _user1 = user_repo.read_one(&UserLogin::new("", "")).await?;
    let _user2 = user_repo.delete(&UserDelete::new(&100)).await;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
