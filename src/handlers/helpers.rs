use actix_identity::Identity;
use actix_web::web;
use crate::database::common::{DbReadMany, DbReadOne};
use crate::database::common::query_parameters::DbQueryParams;
use crate::database::models::audiobook::{AudiobookDisplay, AudiobookGetByIdJoin, AudiobookSearch};
use crate::database::models::chapter::{ChapterDisplay, ChaptersGetByBookId};
use crate::database::models::Id;
use crate::database::models::user::UserGetById;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::handlers::utilities::{get_active_audiobooks, parse_user_id};
use crate::templates::audiobook::AudiobookDetailBase;
use crate::templates::index::IndexBase;

pub async fn get_releases(
    u: Identity,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<Vec<AudiobookDisplay>, AppError> {
    Ok(book_repo
        .read_many(&AudiobookSearch::with_params(
            DbQueryParams::limit(5, 0),
            parse_user_id(u)?,
        ))
        .await?
        .into_iter()
        .map(AudiobookDisplay::from)
        .collect())
}

pub async fn get_audiobook_detail_base(
    audiobook_repo: web::Data<AudiobookRepository>,
    chapter_repo: web::Data<ChapterRepository>,
    user_id: Id,
    audiobook_id: Id,
) -> Result<AudiobookDetailBase, AppError> {
    let audiobook = audiobook_repo
        .read_one(&AudiobookGetByIdJoin::new(user_id, audiobook_id))
        .await?;

    let chapters = chapter_repo
        .read_many(&ChaptersGetByBookId::new(audiobook_id))
        .await?;

    let displayed_chapters: Vec<ChapterDisplay> = chapters
        .into_iter()
        .enumerate()
        .map(|(order, ch)| ChapterDisplay {
            name: ch.name,
            order: order + 1,
            position: ch.position,
        })
        .collect();

    Ok(AudiobookDetailBase {
        audiobook: AudiobookDisplay::from(audiobook),
        chapters: displayed_chapters,
    })
}

pub async fn get_index_base(
    u: Identity,
    user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<IndexBase, AppError> {
    let user = user_repo
        .read_one(&UserGetById::new(&parse_user_id(u)?))
        .await?;

    let mut audiobooks = book_repo
        .read_many(&AudiobookSearch::default(user.id))
        .await?;

    let active_audiobooks = get_active_audiobooks(&audiobooks);
    audiobooks.retain(|a| a.is_finished() || a.is_never_started());
    let audiobooks = audiobooks.into_iter().map(AudiobookDisplay::from).collect();
    let template = IndexBase {
        username: user.name,
        logged_in: true,
        audiobooks,
        active_audiobooks,
    };
    Ok(template)
}

pub async fn get_library(
    u: Identity,
    user_repo: web::Data<UserRepository>,
) -> Result<Vec<AudiobookDisplay>, AppError> {
    Ok(user_repo
        .get_bookmarked(&parse_user_id(u)?)
        .await?
        .into_iter()
        .map(AudiobookDisplay::from)
        .collect())
}

pub async fn get_studio(
    u: Identity,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<Vec<AudiobookDisplay>, AppError> {
    let user_id = parse_user_id(u)?;
    Ok(book_repo
        .read_many(&AudiobookSearch::search_by_author_id(user_id, user_id))
        .await?
        .into_iter()
        .map(AudiobookDisplay::from)
        .collect())
}