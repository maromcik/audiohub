use actix_identity::Identity;
use actix_web::web;
use crate::database::common::{DbReadMany, DbReadOne};
use crate::database::common::query_parameters::{BookState, DbOrder, DbOrderColumn, DbQueryParams};
use crate::database::models::audiobook::{AudiobookDisplay, AudiobookGetByIdJoin, AudiobookSearch};
use crate::database::models::chapter::{Chapter, ChapterDetail, ChapterDisplay, ChaptersGetByBookId};
use crate::database::models::Id;
use crate::database::models::user::UserGetById;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::chapter::repository::ChapterRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::handlers::utilities::{parse_user_id};
use crate::templates::audiobook::AudiobookDetailBase;
use crate::templates::index::IndexBase;

pub async fn get_releases(
    u: Identity,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<Vec<AudiobookDisplay>, AppError> {
    Ok(book_repo
        .read_many(&AudiobookSearch::with_params(
            DbQueryParams::limit(5,
                                 0,
                                 Some(BookState::Fresh(true))),
            parse_user_id(u)?))
        .await?)
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

    let displayed_chapters = get_displayable_chapters(chapter_repo, audiobook_id).await?;

    Ok(AudiobookDetailBase {
        is_liked: audiobook.is_liked,
        audiobook: AudiobookDisplay::from(audiobook),
        chapters: displayed_chapters,
    })
}

pub async fn get_displayable_chapters(chapter_repo: web::Data<ChapterRepository>, audiobook_id: Id) -> Result<Vec<ChapterDisplay>, AppError> {
    let chapters = chapter_repo.read_many(&ChaptersGetByBookId { audiobook_id }).await?;
    Ok(chapters
        .into_iter()
        .enumerate()
        .map(|(order, ch)| ChapterDisplay {
            id: ch.id,
            name: ch.name,
            order: order + 1,
            position: ch.position,
        })
        .collect())
}

pub async fn get_index_base(
    u: Identity,
    user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<IndexBase, AppError> {
    let user = user_repo
        .read_one(&UserGetById::new(&parse_user_id(u)?))
        .await?;

    let audiobooks = book_repo
        .read_many(&AudiobookSearch::with_params(
            DbQueryParams::state(Some(BookState::Fresh(true))), user.id))
        .await?;
    let active_audiobooks = book_repo
        .read_many(&AudiobookSearch::with_params(
            DbQueryParams::new(
                Some(DbOrderColumn::new("ab.edited_at", DbOrder::Desc)),
                None,
                None,
                Some(BookState::Active(true))), user.id))
        .await?;
    let finished_audiobooks = book_repo
        .read_many(&AudiobookSearch::with_params(
            DbQueryParams::new(
                Some(DbOrderColumn::new("ab.edited_at", DbOrder::Desc)),
                None,
                None,
                Some(BookState::Finished(true))), user.id))
        .await?;
    let template = IndexBase {
        username: user.name,
        logged_in: true,
        audiobooks,
        active_audiobooks,
        finished_audiobooks,
    };
    Ok(template)
}

pub async fn get_library(
    u: Identity,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<Vec<AudiobookDisplay>, AppError> {
    Ok(book_repo
        .get_bookmarked(&parse_user_id(u)?)
        .await?)
}

pub async fn get_studio(
    u: Identity,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<Vec<AudiobookDisplay>, AppError> {
    let user_id = parse_user_id(u)?;
    Ok(book_repo
        .read_many(&AudiobookSearch::search_by_author_id(user_id, user_id))
        .await?)
}