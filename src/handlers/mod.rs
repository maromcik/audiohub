pub mod homepage;
pub mod user;
mod audiobook;

pub use crate::handlers::user::add_user as create_user;
pub use crate::handlers::user::login as user_login_page;
pub use crate::handlers::user::login_user as login_user;
pub use crate::handlers::user::register as user_register_page;

pub use crate::handlers::homepage::homepage as index;

pub use crate::handlers::audiobook::new_audiobook_form as add_audiobook;
