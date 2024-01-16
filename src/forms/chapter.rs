use crate::database::models::Id;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ChapterCreateForm {
    pub name: String,
    pub audiobook_id: Id,
    pub position: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChapterCreateAudiobookInfoForm {
    pub audiobook_id: Id,
    pub position: f64,
}
