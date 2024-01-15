use crate::database::models::Id;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use sqlx::postgres::types::PgInterval;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Audiobook {
    pub id: Id,
    // --------------
    pub name: String,
    pub author_id: Id,
    pub genre_id: Id,
    pub length: PgInterval,
    pub file_path: String,
    pub stream_count: i64,
    pub like_count: i64,
    pub overall_rating: i16,
    pub thumbnail: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct AudiobookDetail {
    pub id: Id,
    // --------------
    pub name: String,
    pub author_id: Id,
    pub genre_id: Id,
    pub length: PgInterval,
    pub file_path: String,
    pub stream_count: i64,
    pub like_count: i64,
    pub overall_rating: i16,
    pub thumbnail: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,

    pub username: String,
    pub email: String,
    pub author_name: String,
    pub surname: String,
    pub bio: String,
    pub profile_picture: String,

    pub genre_name: String,
}

#[derive(Debug, Clone)]
pub struct AudiobookSearch {
    pub name: Option<String>,
    pub author_name: Option<String>,
    pub genre_name: Option<String>,
    pub author_id: Option<Id>,
    pub genre_id: Option<Id>,
    pub min_length: Option<PgInterval>,
    pub max_length: Option<PgInterval>,
    pub min_stream_count: Option<i64>,
    pub max_stream_count: Option<i64>,
    pub min_like_count: Option<i64>,
    pub max_like_count: Option<i64>,
    pub min_overall_rating: Option<i16>,
    pub max_overall_rating: Option<i16>,
}

impl AudiobookSearch {
    #[must_use]
    #[inline]
    pub fn new(
        name: Option<&str>,
        author_id: Option<Id>,
        _author_name: Option<&str>,
        genre_id: Option<Id>,
        _genre_name: Option<&str>,
        min_length: Option<PgInterval>,
        max_length: Option<PgInterval>,
        min_stream_count: Option<i64>,
        max_stream_count: Option<i64>,
        min_like_count: Option<i64>,
        max_like_count: Option<i64>,
        min_overall_rating: Option<i16>,
        max_overall_rating: Option<i16>,
    ) -> Self {
        Self {
            name: name.map(|n| n.to_owned()),
            genre_name: name.map(|n| n.to_owned()),
            author_name: name.map(|n| n.to_owned()),
            author_id: author_id.map(|n| n.to_owned()),
            genre_id: genre_id.map(|n| n.to_owned()),
            min_length: min_length.map(|n| n.to_owned()),
            max_length: max_length.map(|n| n.to_owned()),
            min_stream_count: min_stream_count.map(|n| n.to_owned()),
            max_stream_count: max_stream_count.map(|n| n.to_owned()),
            min_like_count: min_like_count.map(|n| n.to_owned()),
            max_like_count: max_like_count.map(|n| n.to_owned()),
            min_overall_rating: min_overall_rating.map(|n| n.to_owned()),
            max_overall_rating: max_overall_rating.map(|n| n.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AudiobookCreate {
    pub name: String,
    pub author_id: Id,
    pub genre_id: Id,
    pub length: PgInterval,
    pub file_path: String,
    pub thumbnail: String,
    pub description: String,
}

impl AudiobookCreate {
    #[must_use]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: &str,
        author_id: &Id,
        genre_id: &Id,
        length: &PgInterval,
        file_path: &str,
        thumbnail: &str,
        description: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            author_id: *author_id,
            genre_id: *genre_id,
            length: length.clone(),
            file_path: file_path.to_owned(),
            thumbnail: thumbnail.to_owned(),
            description: description.to_owned(),
        }
    }
}

pub struct AudiobookUpdate {
    pub id: Id,
    pub name: Option<String>,
    pub author_id: Option<Id>,
    pub genre_id: Option<Id>,
    pub length: Option<PgInterval>,
    pub file_path: Option<String>,
    pub stream_count: Option<i64>,
    pub like_count: Option<i64>,
    pub overall_rating: Option<i16>,
    pub thumbnail: Option<String>,
    pub description: Option<String>,
}

impl AudiobookUpdate {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &Id,
        name: Option<&str>,
        author_id: Option<&Id>,
        genre_id: Option<&Id>,
        length: Option<&PgInterval>,
        file_path: Option<&str>,
        stream_count: Option<&i64>,
        like_count: Option<&i64>,
        overall_rating: Option<&i16>,
        thumbnail: Option<&str>,
        description: Option<&str>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: *id,
            name: name.and_then(change_to_owned),
            author_id: author_id.copied(),
            genre_id: genre_id.copied(),
            length: length.cloned(),
            file_path: file_path.and_then(change_to_owned),
            stream_count: stream_count.copied(),
            like_count: like_count.copied(),
            overall_rating: overall_rating.copied(),
            thumbnail: thumbnail.and_then(change_to_owned),
            description: description.and_then(change_to_owned),
        }
    }

    #[must_use]
    pub const fn update_fields_none(&self) -> bool {
        self.name.is_none()
            && self.author_id.is_none()
            && self.genre_id.is_none()
            && self.length.is_none()
            && self.file_path.is_none()
            && self.stream_count.is_none()
            && self.like_count.is_none()
            && self.overall_rating.is_none()
            && self.description.is_none()
            && self.thumbnail.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct AudiobookDelete {
    pub id: Id,
}

impl AudiobookDelete {
    #[must_use]
    #[inline]
    pub fn new(id: &Id) -> Self {
        Self { id: *id }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct AudiobookGetById {
    pub id: Id,
}

impl AudiobookGetById {
    #[must_use]
    #[inline]
    pub const fn new(id: &Id) -> Self {
        Self { id: *id }
    }
}

#[derive(Debug, Clone)]
pub struct AudiobookMetadataForm {
    pub name: String,
    pub description: String,
    pub genre_id: Id,
}
