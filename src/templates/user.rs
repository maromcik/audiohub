use askama::Template;
use crate::database::models::user::User;

#[derive(Template)]
#[template(path = "user/registration.html")]
pub struct RegistrationTemplate {}

#[derive(Template)]
#[template(path = "user/login.html")]
pub struct LoginTemplate {
    pub message: String,
}


#[derive(Template)]
#[template(path = "user/management.html")]
pub struct UserManagementTemplate {
    pub user: User,
}