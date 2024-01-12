use actix_identity::Identity;
use crate::error::AppError;
use crate::templates::homepage_template::HomepageTemplate;
use actix_web::{get, web, HttpResponse};
use askama::Template;

#[get("/")]
pub async fn homepage(user: Option<Identity>) -> Result<HttpResponse, AppError> {
    let template = HomepageTemplate {};
    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
