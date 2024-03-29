pub mod audiobook;
pub mod chapter;
pub mod genre;
pub mod helpers;
pub mod homepage;
pub mod library;
pub mod rating;
pub mod studio;
pub mod user;
pub mod utilities;

pub use crate::handlers::user::author_content;
pub use crate::handlers::user::author_index;
pub use crate::handlers::user::login as user_login_page;
pub use crate::handlers::user::login_user as user_login;
pub use crate::handlers::user::logout_user as user_logout;
pub use crate::handlers::user::register as user_register_page;
pub use crate::handlers::user::register_user as user_register;
pub use crate::handlers::user::user_manage;

pub use crate::handlers::user::user_manage_form_page;
pub use crate::handlers::user::user_manage_password;
pub use crate::handlers::user::user_manage_password_form;
pub use crate::handlers::user::user_manage_picture;
pub use crate::handlers::user::user_manage_picture_form;

pub use crate::handlers::audiobook::*;
pub use crate::handlers::chapter::*;
pub use crate::handlers::genre::*;
pub use crate::handlers::homepage::*;
pub use crate::handlers::rating::*;
