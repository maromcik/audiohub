use crate::error::AppError;
use crate::templates::homepage_template::HomepageTemplate;
use actix_identity::Identity;
use actix_web::{get, HttpResponse};
use askama::Template;

#[get("/")]
pub async fn homepage(user: Option<Identity>) -> Result<HttpResponse, AppError> {
    let template = match user {
        None => HomepageTemplate {
            username: "None".to_string(),
            logged_in: false,
        },
        Some(u) => HomepageTemplate {
            username: u.id()?,
            logged_in: true,
        },
    };
    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
