
use crate::authorized;
use crate::database::models::Id;
use crate::error::AppError;
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use serde::Deserialize;

use crate::database::models::rating::{ RatingCreate, RatingSearch, UserRatingDisplay};
use crate::database::repositories::rating::repository::RatingRepository;
use crate::forms::rating::RatingCreateForm;

use crate::handlers::utilities::parse_user_id;
use crate::templates::rating::{AudiobookRatingsTemplate, UserRatingTemplate};

#[get("/create/form")]
pub async fn create_rating_form(identity: Option<Identity>) -> Result<HttpResponse, AppError> {
    authorized!(identity);
    todo!()
    // let template = ...
    // Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}

#[post("/audiobook/{book_id}")]
pub async fn create_rating(
    identity: Option<Identity>,
    rating_repo: web::Data<RatingRepository>,
    path: web::Path<(Id)>,
    form: web::Form<RatingCreateForm>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity);
    let user_id = parse_user_id(identity)?;
    let audiobook_id = path.into_inner();
    let rating = rating_repo.create_displayed_rating(&RatingCreate {
            audiobook_id,
            user_id,
            rating: form.rating,
            review: form.review.to_owned(),
        })
        .await?;

    let template = UserRatingTemplate { rating };
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}
#[derive(Deserialize)]
struct OffsetQuery {
    offset: i32,
}

/// returns DISPLAYED_RATINGS_COUNT ratings transformed to html from query param offset
#[get("/audiobook/{id}")]
pub async fn get_ratings_by_audiobook(
    identity: Option<Identity>,
    rating_repo: web::Data<RatingRepository>,
    path: web::Path<(Id,)>,
    query: web::Query<OffsetQuery>
) -> Result<HttpResponse, AppError> {
    authorized!(identity);

    let search_params = RatingSearch::new(Some(path.into_inner().0),None,None,None,None,Some(query.offset));
    let ratings : Vec<UserRatingDisplay> = rating_repo
        .get_ratings_display(&search_params)
        .await?;

    if ratings.len() == 0 {
        return Ok(HttpResponse::PreconditionFailed().finish());
    }
    let template = AudiobookRatingsTemplate {ratings,};
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))

}
