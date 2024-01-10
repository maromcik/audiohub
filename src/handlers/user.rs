use crate::database::common::error::{BusinessLogicErrorKind, DbResultMultiple};
use crate::database::common::{DbDelete, DbReadOne};
use crate::database::models::user::{UserDelete, UserLogin};
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::templates::user::Index;
use actix_web::{get, web, HttpResponse};
use askama::Template;

#[get("/index")]
pub async fn index(user_repo: web::Data<UserRepository>) -> Result<HttpResponse, AppError> {
    let template = Index {};
    let body = template.render()?;
    let _user1 = user_repo.read_one(&UserLogin::new("", "")).await?;
    let _user2 = user_repo.delete(&UserDelete::new(&100)).await;

    // match _user2 {
    //     Ok(_) => {Ok(HttpResponse::Ok().content_type("text/html").body(body))}
    //     Err(e) => {
    //         match e.business_error.error_kind {
    //             BusinessLogicErrorKind::UserDoesNotExist => {}
    //             BusinessLogicErrorKind::UserDeleted => {}
    //             BusinessLogicErrorKind::UserPasswordDoesNotMatch => {}
    //             BusinessLogicErrorKind::UserUpdateParametersEmpty => {}
    //             BusinessLogicErrorKind::AudiobookDoesNotExist => {}
    //             BusinessLogicErrorKind::AudiobookDeleted => {}
    //             BusinessLogicErrorKind::AudiobookUpdateParametersEmpty => {}
    //             BusinessLogicErrorKind::RatingDoesNotExist => {}
    //             BusinessLogicErrorKind::RatingDeleted => {}
    //             BusinessLogicErrorKind::RatingUpdateParametersEmpty => {}
    //             BusinessLogicErrorKind::ChapterDoesNotExist => {}
    //             BusinessLogicErrorKind::ChapterDeleted => {}
    //             BusinessLogicErrorKind::ChapterUpdateParametersEmpty => {}
    //             BusinessLogicErrorKind::GenreDeleted => {}
    //             BusinessLogicErrorKind::GenreDoesNotExist => {}
    //             BusinessLogicErrorKind::GenreUpdateParametersEmpty => {}
    //             BusinessLogicErrorKind::DatabaseError => {}
    //             BusinessLogicErrorKind::MigrationError => {}
    //             BusinessLogicErrorKind::UniqueConstraintError => {}
    //             BusinessLogicErrorKind::NotNullError => {}
    //             BusinessLogicErrorKind::ForeignKeyError => {}
    //         }
    //     }
    // }
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
