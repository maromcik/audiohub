use crate::database::models::Id;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RatingCreateForm {
    pub audiobook_id: Id,
    pub user_id: Id,
    pub rating: i16,
    pub review: Option<String>,
}
