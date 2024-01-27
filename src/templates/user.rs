use crate::database::models::user::UserDisplay;
use askama::Template;

const WEAK_PASSWORD_MESSAGE: &str = "Weak password! Password must contain at least one from each: {lower case character, upper case character, number, special character} and must be at least 6 characters long";

#[derive(Template, Default)]
#[template(path = "user/registration.html")]
pub struct RegistrationTemplate {
    pub message: String,
}


impl RegistrationTemplate {
    pub fn weak_password() -> Self {
        Self {
            message: WEAK_PASSWORD_MESSAGE.to_owned()
        }
    }
}

#[derive(Template, Default)]
#[template(path = "user/login.html")]
pub struct LoginTemplate {
    pub message: String,
    pub return_url: String,
}

#[derive(Template, Default)]
#[template(path = "user/password.html")]
pub struct UserManagePasswordTemplate {
    pub message: String,
    pub success: bool
}

impl UserManagePasswordTemplate {
    pub fn weak_password() -> Self {
        Self {
            success: false,
            message: WEAK_PASSWORD_MESSAGE.to_owned()
        }
    }
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

#[derive(Template, Default)]
#[template(path = "user/profile_picture_update.html")]
pub struct UserManageProfilePictureFormTemplate {
    pub message: String,
}

#[derive(Template)]
#[template(path = "user/profile_picture.html")]
pub struct UserManageProfilePictureTemplate {
    pub user: UserDisplay,
}

#[derive(Template, Default)]
#[template(path = "user/simple_responses/success_update.html")]
pub struct UserManageProfileSuccessfulUpdate {}

#[derive(Template, Default)]
#[template(path = "user/simple_responses/success_update_password.html")]
pub struct UserManageProfileSuccessfulUpdatePassword {}

#[derive(Template)]
#[template(path = "user/profile_user_form.html")]
pub struct UserManageProfileUserFormTemplate {
    pub user: UserDisplay,
    pub message: String,
    pub success: bool
}
