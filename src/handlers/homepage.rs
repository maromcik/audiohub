use crate::database::common::{DbReadMany, DbReadOne};
use crate::database::models::audiobook::AudiobookSearch;
use crate::database::models::user::UserGetById;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::handlers::utilities::parse_user_id;
use crate::templates::index::{IndexTemplate, IndexContentTemplate};
use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};
use askama::Template;

#[get("/")]
pub async fn index(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let books = book_repo
        .read_many(&AudiobookSearch::default())
        .await?;

    let template = match identity {
        None => IndexTemplate {
            username: "None".to_string(),
            logged_in: false,
            audiobooks: books,
        },
        Some(u) => {
            let user = user_repo
                .read_one(&UserGetById::new(&parse_user_id(u)?))
                .await?;
            IndexTemplate {
                username: user.name,
                logged_in: true,
                audiobooks: books,
            }
        }
    };
    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/home")]
pub async fn index_content(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let books = book_repo
        .read_many(&AudiobookSearch::default())
        .await?;

    let template = match identity {
        None => IndexContentTemplate {
            username: "None".to_string(),
            logged_in: false,
            audiobooks: books,
        },
        Some(u) => {
            let user = user_repo
                .read_one(&UserGetById::new(&parse_user_id(u)?))
                .await?;
            IndexContentTemplate {
                username: user.name,
                logged_in: true,
                audiobooks: books,
            }
        }
    };
    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}