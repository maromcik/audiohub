use askama::Template;

#[derive(Template)]
#[template(path = "user/registration.html")]
pub struct RegistrationTemplate {}

#[derive(Template)]
#[template(path = "user/login.html")]
pub struct LoginTemplate {}
