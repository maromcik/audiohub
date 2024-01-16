use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use serde::Deserialize;
#[derive(Deserialize, Debug, Clone)]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub surname: String,
}


#[derive(Debug, MultipartForm)]
pub struct ProfilePictureUploadForm {
    #[multipart(rename = "picture")]
    pub picture: TempFile,
}