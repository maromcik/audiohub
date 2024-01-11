pub mod homepage;
pub mod user;

pub use crate::handlers::user::add_user as create_user;
pub use crate::handlers::user::login as user_login;
pub use crate::handlers::user::register as user_register;

pub use crate::handlers::homepage::homepage as index;
