pub mod audiobook;
pub mod chapter;
pub mod genre;
pub mod homepage;
pub mod library;
pub mod rating;
pub mod studio;
pub mod user;
pub mod utilities;
pub mod helpers;

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

pub use crate::handlers::homepage::index;
pub use crate::handlers::homepage::index_content;

pub use crate::handlers::audiobook::create_audiobook;
pub use crate::handlers::audiobook::create_audiobook_page;
pub use crate::handlers::audiobook::get_audiobook;
pub use crate::handlers::audiobook::manage_audiobook_content;
pub use crate::handlers::audiobook::manage_audiobook;
pub use crate::handlers::audiobook::remove_audiobook;
pub use crate::handlers::audiobook::remove_audiobook_in_studio;
pub use crate::handlers::audiobook::search;
pub use crate::handlers::audiobook::set_active_audiobook;
pub use crate::handlers::audiobook::upload_audiobook;
pub use crate::handlers::audiobook::upload_audiobook_form;

pub use crate::handlers::chapter::create_chapter;

pub use crate::handlers::genre::get_audiobooks_by_genre;
pub use crate::handlers::genre::get_genres_page;
