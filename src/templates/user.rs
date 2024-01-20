use crate::database::models::user::User;
use askama::Template;

#[derive(Template)]
#[template(path = "user/registration.html")]
pub struct RegistrationTemplate {}

#[derive(Template)]
#[template(path = "user/login.html")]
pub struct LoginTemplate {
    pub message: String,
}

#[derive(Template)]
#[template(path = "user/password.html")]
pub struct UserManagePasswordTemplate {}

#[derive(Template)]
#[template(path = "user-manage.html")]
pub struct UserManageProfilePageTemplate {
    pub user: User,
}

#[derive(Template)]
#[template(path = "user/profile.html")]
pub struct UserManageProfileContentTemplate {
    pub user: User,
}

#[derive(Template)]
#[template(path = "user/profile_picture_update.html")]
pub struct UserManageProfilePictureFormTemplate {}

#[derive(Template)]
#[template(path = "user/profile_picture.html")]
pub struct UserManageProfilePictureTemplate {
    pub user: User,
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
    pub user: User,
}
