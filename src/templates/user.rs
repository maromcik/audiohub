use crate::database::models::user::UserDisplay;
use askama::Template;

#[derive(Template)]
#[template(path = "user/registration.html")]
pub struct RegistrationTemplate {
    pub message: String,
}

#[derive(Template)]
#[template(path = "user/login.html")]
pub struct LoginTemplate {
    pub message: String,
    pub return_url: String
}

#[derive(Template)]
#[template(path = "user/password.html")]
pub struct UserManagePasswordTemplate {
    pub message: String,
    pub success: bool
}

#[derive(Template)]
#[template(path = "user-manage.html")]
pub struct UserManageProfilePageTemplate {
    pub user: UserDisplay,
    pub message: String,
    pub success: bool
}

#[derive(Template)]
#[template(path = "user/profile.html")]
pub struct UserManageProfileContentTemplate {
    pub user: UserDisplay,
    pub message: String,
    pub success: bool
}

#[derive(Template)]
#[template(path = "user/profile_picture_update.html")]
pub struct UserManageProfilePictureFormTemplate {
    pub message: String,
}

#[derive(Template)]
#[template(path = "user/profile_picture.html")]
pub struct UserManageProfilePictureTemplate {
    pub user: UserDisplay,
}

#[derive(Template)]
#[template(path = "user/simple_responses/success_update.html")]
pub struct UserManageProfileSuccessfulUpdate {}

#[derive(Template)]
#[template(path = "user/simple_responses/success_update_password.html")]
pub struct UserManageProfileSuccessfulUpdatePassword {}

#[derive(Template)]
#[template(path = "user/profile_user_form.html")]
pub struct UserManageProfileUserFormTemplate {
    pub user: UserDisplay,
    pub message: String,
    pub success: bool
}
