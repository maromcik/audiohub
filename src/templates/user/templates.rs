use askama::Template;

#[derive(Template)]
#[template(path = "registration.html")]
pub struct RegistrationTemplate {}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {}