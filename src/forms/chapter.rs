use serde::Deserialize;
use crate::database::models::Id;

#[derive(Debug, Clone, Deserialize)]
pub struct ChapterCreateForm {
    pub name: String,
    pub audiobook_id: Id,
    pub length: String,
    pub sequential_number: i32
}