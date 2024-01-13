use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AudiobookCreateForm {
    pub name: String,
    pub description: String,
    pub genre_name: String,
}
#[derive(Debug, MultipartForm)]
pub struct AudiobookUploadForm {
    #[multipart(rename = "thumbnail")]
    pub thumbnail: TempFile,
    #[multipart(rename = "file")]
    pub audio_file: TempFile,
}
