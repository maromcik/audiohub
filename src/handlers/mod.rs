mod audiobook;
pub mod homepage;
pub mod user;
mod audiobook;

pub use crate::handlers::user::login as user_login_page;
pub use crate::handlers::user::login_user as user_login;
pub use crate::handlers::user::register as user_register_page;
pub use crate::handlers::user::register_user as user_register;

pub use crate::handlers::homepage::index;

pub use crate::handlers::audiobook::new_audiobook_form as add_audiobook;
