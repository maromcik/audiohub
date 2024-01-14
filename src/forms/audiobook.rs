use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use serde::Deserialize;
use crate::database::models::Id;

#[derive(Debug, Clone, Deserialize)]
pub struct AudiobookCreateForm {
    pub name: String,
    pub description: String,
    pub genre_id: Id,
}
#[derive(Debug, MultipartForm)]
pub struct AudiobookUploadForm {
    #[multipart(rename = "thumbnail")]
    pub thumbnail: TempFile,
    #[multipart(rename = "file")]
    pub audio_file: TempFile,
}
