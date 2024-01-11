use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::templates::homepage_template::HomepageTemplate;
use actix_web::{get, web, HttpResponse};
use askama::Template;

#[get("/")]
pub async fn homepage(user_repo: web::Data<UserRepository>) -> Result<HttpResponse, AppError> {
    let template = HomepageTemplate {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
