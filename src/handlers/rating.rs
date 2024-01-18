use crate::authorized;
use crate::database::common::{DbCreate, DbReadMany};
use crate::database::models::Id;
use crate::error::AppError;
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, HttpResponse};

use crate::database::models::rating::{RatingCreate, RatingsGetByBookId};
use crate::database::repositories::rating::repository::RatingRepository;
use crate::forms::rating::RatingCreateForm;

#[get("/create/form")]
pub async fn create_rating_form(identity: Option<Identity>) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    // let body = template.render()?;
    // Ok(HttpResponse::Ok().content_type("text/html").body(body))
    todo!()
}

#[post("/create")]
pub async fn create_rating(
    identity: Option<Identity>,
    rating_repo: web::Data<RatingRepository>,
    form: web::Form<RatingCreateForm>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let _rating = rating_repo
        .create(&RatingCreate {
            audiobook_id: form.audiobook_id,
            user_id: form.user_id,
            rating: form.rating,
            review: form.review.to_owned(),
        })
        .await?;

    // let template =
    // let body = template.render()?;
    // Ok(HttpResponse::Ok().content_type("text/html").body(body))
    todo!()
}

#[get("/audiobook/{id}")]
pub async fn get_ratings_by_audiobook(
    identity: Option<Identity>,
    rating_repo: web::Data<RatingRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    let _ratings = rating_repo
        .read_many(&RatingsGetByBookId::new(path.into_inner().0))
        .await?;
    // let template =
    // let body = template.render()?;
    // Ok(HttpResponse::Ok().content_type("text/html").body(body))
    todo!()
}
