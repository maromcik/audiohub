use crate::database::common::HasDeletedAt;
use crate::database::models::Id;
use chrono::{DateTime, Utc};
use serde::Deserialize;

pub const DISPLAYED_RATINGS_COUNT: i32 = 5;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Rating {
    pub id: Id,
    pub audiobook_id: Id,
    pub user_id: Id,
    pub rating: i16,
    pub review: Option<String>,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl HasDeletedAt for Rating {
    fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

#[derive(Debug, Clone, Default)]
pub struct RatingSearch {
    pub audiobook_id: Option<Id>,
    pub user_id: Option<Id>,
    pub min_rating: Option<i16>,
    pub max_rating: Option<i16>,
    pub review: Option<String>,
    pub offset: Option<i32>,
}

impl RatingSearch {
    #[must_use]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        audiobook_id: Option<Id>,
        user_id: Option<Id>,
        min_rating: Option<i16>,
        max_rating: Option<i16>,
        review: Option<&str>,
        offset: Option<i32>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            audiobook_id,
            user_id,
            min_rating,
            max_rating,
            review: review.and_then(change_to_owned),
            offset,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RatingsGetByBookId {
    pub audiobook_id: Id,
}

impl RatingsGetByBookId {
    #[allow(dead_code)]
    pub fn new(id: Id) -> Self {
        Self { audiobook_id: id }
    }
}

#[derive(Debug, Clone)]
pub struct RatingCreate {
    pub audiobook_id: Id,
    pub user_id: Id,
    pub rating: i16,
    pub review: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RatingUpdate {
    pub id: Id,
    pub rating: i16,
    pub review: Option<String>,
}

pub struct RatingGetById {
    pub id: Id,
}

#[derive(Debug, Clone)]
pub struct UserRatingDisplay {
    pub user_id: Id,
    pub book_id: Id,
    pub user_name: String,
    pub user_surname: String,
    pub user_thumbnail: Option<String>,
    pub rating: i16,
    pub review: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct RatingSummaryDisplay {
    pub all_ratings_count: i64,
    pub star_count: Vec<i64>,
    pub overall_rating: f64,
}
