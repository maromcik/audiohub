use crate::database::common::PoolHandler;
use crate::database::common::{DbPoolHandler, DbRepository};
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::database::repositories::genre::repository::GenreRepository;
use crate::database::repositories::rating::repository::RatingRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::handlers::audiobook::{change_like, releases_content, releases_page};
use crate::handlers::rating::{create_rating, create_rating_form, get_ratings_by_audiobook};
use crate::handlers::*;
use actix_files::Files as ActixFiles;
use actix_web::web;
use actix_web::web::ServiceConfig;
use sqlx::PgPool;

pub fn configure_webapp(pool: &PgPool) -> Box<dyn FnOnce(&mut ServiceConfig)> {
    let user_repository = UserRepository::new(PoolHandler::new(pool.clone()));
    let audiobook_repository = AudiobookRepository::new(PoolHandler::new(pool.clone()));
    let chapter_repository = ChapterRepository::new(PoolHandler::new(pool.clone()));
    let genre_repository = GenreRepository::new(PoolHandler::new(pool.clone()));
    let rating_repository = RatingRepository::new(PoolHandler::new(pool.clone()));

    let user_scope = web::scope("user")
        .service(user_login_page)
        .service(user_login)
        .service(user_register_page)
        .service(user_register)
        .service(user_logout)
        .service(user_manage_form)
        .service(user_manage_password_form)
        .service(user_manage_picture_form)
        .service(user_manage)
        .service(user_manage_picture)
        .service(user_manage_password);

    let audiobook_scope = web::scope("audiobook")
        .app_data(web::Data::new(audiobook_repository.clone()))
        .app_data(web::Data::new(genre_repository.clone()))
        .app_data(web::Data::new(chapter_repository.clone()))
        .service(create_audiobook)
        .service(upload_audiobook)
        .service(create_audiobook_form)
        .service(upload_audiobook_form)
        .service(get_audiobook)
        .service(releases_content)
        .service(releases_page)
        .service(remove_audiobook)
        .service(change_like)
        .service(search)
        .service(set_active_audiobook)
        .service(remove_active_audiobook);

    let chapter_scope = web::scope("chapter")
        .app_data(web::Data::new(chapter_repository.clone()))
        .service(create_chapter_form)
        .service(create_chapter);

    let genre_scope = web::scope("genre")
        .app_data(web::Data::new(genre_repository.clone()))
        .app_data(web::Data::new(audiobook_repository.clone()))
        .service(get_genres)
        .service(get_audiobooks_by_genre);

    let rating_scope = web::scope("rating")
        .app_data(web::Data::new(rating_repository.clone()))
        .service(create_rating)
        .service(create_rating_form)
        .service(get_ratings_by_audiobook);

    Box::new(move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(user_repository.clone()))
            .app_data(web::Data::new(audiobook_repository.clone()))
            .service(index)
            .service(index_content)
            .service(user_scope)
            .service(genre_scope)
            .service(audiobook_scope)
            .service(chapter_scope)
            .service(rating_scope)
            .service(library::index)
            .service(library::get_content)
            .service(ActixFiles::new("/media", "./media").prefer_utf8(true));
    })
}
