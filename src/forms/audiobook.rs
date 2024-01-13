use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AudiobookCreateForm {
    pub name: String,
    pub description: String,
    pub genre_name: String,
}
