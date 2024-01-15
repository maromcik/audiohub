pub mod audiobook;
pub mod chapter;
pub mod homepage;
pub mod user;
mod utilities;

pub use crate::handlers::user::login as user_login_page;
pub use crate::handlers::user::login_user as user_login;
pub use crate::handlers::user::register as user_register_page;
pub use crate::handlers::user::register_user as user_register;

pub use crate::handlers::homepage::index;

pub use crate::handlers::audiobook::create_audiobook;
pub use crate::handlers::audiobook::create_audiobook_form;
pub use crate::handlers::audiobook::get_audiobook;
pub use crate::handlers::audiobook::releases;
pub use crate::handlers::audiobook::upload_audiobook;
pub use crate::handlers::audiobook::upload_audiobook_form;

pub use crate::handlers::chapter::create_chapter_form;
pub use crate::handlers::chapter::get_chapters_by_book;
