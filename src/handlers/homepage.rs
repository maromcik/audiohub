use crate::error::AppError;
use crate::templates::index::IndexTemplate;
use actix_identity::Identity;
use actix_web::{get, HttpResponse};
use askama::Template;

#[get("/")]
pub async fn index(user: Option<Identity>) -> Result<HttpResponse, AppError> {
    let template = match user {
        None => IndexTemplate {
            username: "None".to_string(),
            logged_in: false,
        },
        Some(u) => IndexTemplate {
            username: u.id()?,
            logged_in: true,
        },
    };
    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
