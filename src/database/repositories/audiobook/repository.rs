use crate::database::common::error::BackendErrorKind::{
    AudiobookDeleted, AudiobookDoesNotExist, AudiobookUpdateParametersEmpty,
};
use crate::database::common::error::{BackendError, DbError, DbResultMultiple, DbResultSingle};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler,
};
use async_trait::async_trait;

use sqlx::{Postgres, Transaction};

use crate::database::models::audiobook::{Audiobook, AudiobookCreate, AudiobookDelete, AudiobookDetail, AudiobookGetById, AudiobookSearch, AudiobookUpdate};

#[derive(Clone)]
pub struct AudiobookRepository {
    pool_handler: PoolHandler,
}

impl AudiobookRepository {
    pub async fn get_audiobook<'a>(
        params: AudiobookGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Audiobook>> {
        let query = sqlx::query_as!(
            Audiobook,
            r#"
            SELECT * FROM "Audiobook"
            WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        if let Some(book) = query {
            return Ok(Option::from(book));
        }

        Err(DbError::from(BackendError::new(AudiobookDoesNotExist)))
    }

    pub fn audiobook_is_correct(audiobook: Option<Audiobook>) -> DbResultSingle<Audiobook> {
        if let Some(audiobook) = audiobook {
            if audiobook.deleted_at.is_none() {
                return Ok(audiobook);
            }
            return Err(DbError::from(BackendError::new(AudiobookDeleted)));
        }

        Err(DbError::from(BackendError::new(AudiobookDoesNotExist)))
    }
}

#[async_trait]
impl DbRepository for AudiobookRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    async fn disconnect(&self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbReadOne<AudiobookGetById, Audiobook> for AudiobookRepository {
    /// Login the user with provided parameters, if the user does not exist, is deleted or the
    /// passwords don't match, return the error about combination of email/password not working
    async fn read_one(&self, params: &AudiobookGetById) -> DbResultSingle<Audiobook> {
        let maybe_audiobook = sqlx::query_as!(
            Audiobook,
            r#"
            SELECT * FROM "Audiobook"
            WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(&self.pool_handler.pool)
        .await?;

        let audiobook = AudiobookRepository::audiobook_is_correct(maybe_audiobook)?;
        Ok(audiobook)
    }
}

// #[async_trait]
// impl DbReadMany<AudiobookSearch, Audiobook> for AudiobookRepository {
//     async fn read_many(&self, params: &AudiobookSearch) -> DbResultMultiple<Audiobook> {
//         let audiobooks = sqlx::query_as!(
//             Audiobook,
//             r#"
//             SELECT * FROM "Audiobook"
//             WHERE
//                 (name = $1 OR $1 IS NULL)
//                 AND (author_id = $2 OR $2 IS NULL)
//                 AND (genre_id = $3 OR $3 IS NULL)
//                 AND (like_count >= $4 OR $4 IS NULL)
//                 AND (like_count <= $5 OR $5 IS NULL)
//                 AND (length >= $6 OR $6 IS NULL)
//                 AND (length <= $7 OR $7 IS NULL)
//                 AND (stream_count >= $8 OR $8 IS NULL)
//                 AND (stream_count <= $9 OR $9 IS NULL)
//                 AND (overall_rating >= $10 OR $10 IS NULL)
//                 AND (overall_rating <= $11 OR $11 IS NULL)
//             "#,
//             params.name,
//             params.author_id,
//             params.genre_id,
//             params.min_like_count,
//             params.max_like_count,
//             params.min_length,
//             params.max_length,
//             params.min_stream_count,
//             params.max_stream_count,
//             params.min_overall_rating,
//             params.max_overall_rating,
//         )
//         .fetch_all(&self.pool_handler.pool)
//         .await?;
//         Ok(audiobooks)
//     }
// }

#[async_trait]
impl DbReadMany<AudiobookSearch, AudiobookDetail> for AudiobookRepository {
    async fn read_many(&self, params: &AudiobookSearch) -> DbResultMultiple<AudiobookDetail> {
        let audiobooks = sqlx::query_as!(
            AudiobookDetail,
            r#"
            SELECT
                a.id,
                a.name,
                a.description,
                a.length,
                a.file_path,
                a.thumbnail,
                a.overall_rating,
                a.stream_count,
                a.like_count,
                a.created_at,
                a.edited_at,

                a.author_id,
                u.name AS author_name,
                u.surname,
                u.username,
                u.email,
                u.profile_picture,
                u.bio,

                a.genre_id,
                g.name AS genre_name

            FROM
                "User" AS u
                    INNER JOIN
                "Audiobook" AS a
                    ON a.author_id = u.id
                    INNER JOIN
                "Genre" AS g
                    ON a.genre_id = g.id
            WHERE
                a.deleted_at IS NULL
                AND u.deleted_at IS NULL
                AND g.deleted_at IS NULL
                AND (a.name = $1 OR $1 IS NULL)
                AND (author_id = $2 OR $2 IS NULL)
                AND (genre_id = $3 OR $3 IS NULL)
                AND (like_count >= $4 OR $4 IS NULL)
                AND (like_count <= $5 OR $5 IS NULL)
                AND (length >= $6 OR $6 IS NULL)
                AND (length <= $7 OR $7 IS NULL)
                AND (stream_count >= $8 OR $8 IS NULL)
                AND (stream_count <= $9 OR $9 IS NULL)
                AND (overall_rating >= $10 OR $10 IS NULL)
                AND (overall_rating <= $11 OR $11 IS NULL)
                AND (u.name = $12 OR $12 IS NULL)
                AND (g.name = $13 OR $13 IS NULL)
            ORDER BY
                a.created_at
                    DESC;
            "#,
            params.name,
            params.author_id,
            params.genre_id,
            params.min_like_count,
            params.max_like_count,
            params.min_length,
            params.max_length,
            params.min_stream_count,
            params.max_stream_count,
            params.min_overall_rating,
            params.max_overall_rating,
            params.author_name,
            params.genre_name
        )
            .fetch_all(&self.pool_handler.pool)
            .await?;
        Ok(audiobooks)
    }
}

#[async_trait]
impl DbCreate<AudiobookCreate, Audiobook> for AudiobookRepository {
    async fn create(&self, params: &AudiobookCreate) -> DbResultSingle<Audiobook> {
        let book = sqlx::query_as!(
            Audiobook,
            r#"
            INSERT INTO "Audiobook" (name, author_id, genre_id, length, file_path, thumbnail, description)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            params.name,
            params.author_id,
            params.genre_id,
            params.length,
            params.file_path,
            params.thumbnail,
            params.description
        )
        .fetch_one(&self.pool_handler.pool)
        .await?;

        Ok(book)
    }
}

#[async_trait]
impl DbUpdate<AudiobookUpdate, Audiobook> for AudiobookRepository {
    async fn update(&self, params: &AudiobookUpdate) -> DbResultMultiple<Audiobook> {
        if params.update_fields_none() {
            return Err(DbError::from(BackendError::new(
                AudiobookUpdateParametersEmpty,
            )));
        }

        let mut transaction = self.pool_handler.pool.begin().await?;
        let audiobook = AudiobookRepository::get_audiobook(
            AudiobookGetById { id: params.id },
            &mut transaction,
        )
        .await?;
        let validated_audiobook = AudiobookRepository::audiobook_is_correct(audiobook)?;
        let updated_audio_books = sqlx::query_as!(
            Audiobook,
            r#"
            UPDATE "Audiobook"
            SET
                name = COALESCE($1, name),
                author_id = COALESCE($2, author_id),
                genre_id = COALESCE($3, genre_id),
                length = COALESCE($4, length),
                file_path = COALESCE($5, file_path),
                stream_count = COALESCE($6, stream_count),
                like_count = COALESCE($7, like_count),
                overall_rating = COALESCE($8, overall_rating),
                thumbnail = COALESCE($9, thumbnail),
                description = COALESCE($10, thumbnail),
                edited_at = current_timestamp
            WHERE id = $11
            RETURNING *
            "#,
            params.name,
            params.author_id,
            params.genre_id,
            params.length,
            params.file_path,
            params.stream_count,
            params.like_count,
            params.overall_rating,
            params.thumbnail,
            params.description,
            validated_audiobook.id
        )
        .fetch_all(transaction.as_mut())
        .await?;
        transaction.commit().await?;

        Ok(updated_audio_books)
    }
}

#[async_trait]
impl DbDelete<AudiobookDelete, Audiobook> for AudiobookRepository {
    async fn delete(&self, params: &AudiobookDelete) -> DbResultMultiple<Audiobook> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let book_query = AudiobookRepository::get_audiobook(
            AudiobookGetById { id: params.id },
            &mut transaction,
        )
        .await?;

        let _ = AudiobookRepository::audiobook_is_correct(book_query.clone())?;

        let books = sqlx::query_as!(
            Audiobook,
            r#"
            UPDATE "Audiobook" SET
                name = $1,
                deleted_at = current_timestamp,
                edited_at = current_timestamp
            WHERE id = $1
            RETURNING *
            "#,
            params.id,
        )
        .fetch_all(transaction.as_mut())
        .await?;
        transaction.commit().await?;

        Ok(books)
    }
}
