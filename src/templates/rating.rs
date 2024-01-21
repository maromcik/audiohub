use askama::Template;
use crate::database::models::rating::UserRatingDisplay;

#[derive(Template)]
#[template(path = "rating/rating.html")]
pub struct UserRatingTemplate {
    pub rating: UserRatingDisplay
}


#[derive(Template)]
#[template(path = "rating/book-ratings.html")]
pub struct AudiobookRatingsTemplate {
    pub ratings: Vec<UserRatingDisplay>,
}
