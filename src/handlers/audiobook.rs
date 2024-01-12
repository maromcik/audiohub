use actix_web::{get, HttpResponse, web};
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::error::AppError;
use crate::templates::audiobook::NewAudiobookForm;
use askama::Template;

#[get("/new")]
pub async fn new_audiobook_form(user_repo: web::Data<AudiobookRepository>) -> Result<HttpResponse, AppError> {
    let template = NewAudiobookForm {};
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}