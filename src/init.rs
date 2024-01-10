use crate::database::common::PoolHandler;
use crate::database::common::{DbPoolHandler, DbRepository};
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::database::repositories::genre::repository::GenreRepository;
use crate::database::repositories::rating::repository::RatingRepository;
use crate::database::repositories::user::repository::UserRepository;
use actix_web::web;
use actix_web::web::ServiceConfig;
use sqlx::PgPool;
use std::sync::Arc;

pub fn configure_webapp(pool: &Arc<PgPool>) -> Box<dyn FnOnce(&mut ServiceConfig)> {
    let user_repository = UserRepository::new(PoolHandler::new(pool.clone()));
    let audiobook_repository = AudiobookRepository::new(PoolHandler::new(pool.clone()));
    let chapter_repository = ChapterRepository::new(PoolHandler::new(pool.clone()));
    let genre_repository = GenreRepository::new(PoolHandler::new(pool.clone()));
    let rating_repository = RatingRepository::new(PoolHandler::new(pool.clone()));

    let user_scope = web::scope("user").app_data(web::Data::new(user_repository.clone()));
    // .service(user_get);
    // .service(user_post);

    let audiobook_scope =
        web::scope("audiobook").app_data(web::Data::new(audiobook_repository.clone()));

    let chapter_scope = web::scope("chapter").app_data(web::Data::new(chapter_repository.clone()));

    let genre_scope = web::scope("genre").app_data(web::Data::new(genre_repository.clone()));

    let rating_scope = web::scope("rating").app_data(web::Data::new(rating_repository.clone()));

    Box::new(|cfg: &mut ServiceConfig| {
        cfg.service(user_scope)
            .service(genre_scope)
            .service(audiobook_scope)
            .service(chapter_scope)
            .service(rating_scope);
    })
}