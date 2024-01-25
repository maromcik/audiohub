use crate::database::models::rating::{RatingSummaryDisplay, UserRatingDisplay};
use crate::database::models::Id;
use askama::Template;

#[derive(Template)]
#[template(path = "rating/rating.html")]
pub struct UserRatingTemplate {
    pub rating: UserRatingDisplay,
}

#[derive(Template)]
#[template(path = "rating/book-ratings.html")]
pub struct AudiobookRatingsTemplate {
    pub ratings: Vec<UserRatingDisplay>,
}

#[derive(Template)]
#[template(path = "rating/deleted-rating.html")]
pub struct DeletedRatingTemplate {}

#[derive(Template)]
#[template(path = "rating/my-rating.html")]
pub struct MyRatingTemplate {
    pub rating: UserRatingDisplay,
}

#[derive(Template)]
#[template(path = "rating/rating-summary.html")]
pub struct RatingSummaryTemplate {
    pub summary: RatingSummaryDisplay,
    pub audiobook_id: Id,
}
