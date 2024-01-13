use serde::Deserialize;
#[derive(Deserialize, Debug, Clone)]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub surname: String,
}
