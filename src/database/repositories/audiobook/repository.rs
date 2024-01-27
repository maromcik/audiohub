use crate::database::common::error::BackendErrorKind::{
    AudiobookDeleted, AudiobookDoesNotExist, AudiobookUpdateParametersEmpty,
};
use crate::database::common::error::{BackendError, DbError, DbResultMultiple, DbResultSingle, EntityError};
use crate::database::common::{DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler};
use async_trait::async_trait;

use crate::database::common::utilities::generate_query_param_string;
use crate::database::models::active_audiobook::{
    ActiveAudiobook, PlayedAudiobook, PlayedAudiobookDb, RemoveActiveAudiobook, SetActiveAudiobook,
};
use sqlx::{Postgres, Transaction};

use crate::database::models::audiobook::{Audiobook, AudiobookCreate, AudiobookDelete, AudiobookDetail, AudiobookDisplay, AudiobookGetById, AudiobookGetByIdJoin, AudiobookQuickSearch, AudiobookSearch, AudiobookUpdate};
use crate::database::models::Id;
use crate::database::common::utilities::entity_is_correct;

#[derive(Clone)]
pub struct AudiobookRepository {
    pool_handler: PoolHandler,
}

impl AudiobookRepository {
    pub async fn get_audiobook<'a>(
        params: &AudiobookGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Audiobook> {
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
        entity_is_correct(query, EntityError::new(AudiobookDeleted, AudiobookDoesNotExist))
    }

    pub async fn increment_stream_count<'a>(book_id: &Id, transaction_handle: &mut Transaction<'a, Postgres>) -> DbResultSingle<()> {
        sqlx::query!(
            r#"
            UPDATE "Audiobook"
            SET stream_count = stream_count + 1
            WHERE id = $1
            "#,
            book_id,
        ).execute(transaction_handle.as_mut()).await?;
        Ok(())
    }

    pub async fn quick_search(&self, query: &str) -> DbResultMultiple<AudiobookQuickSearch> {
        let mut comparison_string: String = "%".to_owned();
        comparison_string.push_str(query);
        comparison_string.push('%');

        let results = sqlx::query_as!(
            AudiobookQuickSearch,
            r#"
            SELECT id, name FROM "Audiobook"
            WHERE name ILIKE $1
            LIMIT 5
            "#,
            comparison_string
        )
            .fetch_all(&self.pool_handler.pool)
            .await?;

        Ok(results)
    }

    pub async fn remove_active_audiobook(
        &self,
        params: &RemoveActiveAudiobook,
    ) -> DbResultSingle<ActiveAudiobook> {
        let removed_active_audiobook = sqlx::query_as!(
            ActiveAudiobook,
            r#"
            DELETE FROM "Active_Audiobook"
            WHERE user_id = $1 AND audiobook_id = $2
            RETURNING *
            "#,
            params.user_id,
            params.audiobook_id,
        )
            .fetch_one(&self.pool_handler.pool)
            .await?;

        Ok(removed_active_audiobook)
    }

    pub async fn set_active_audiobook(
        &self,
        params: &SetActiveAudiobook,
    ) -> DbResultSingle<ActiveAudiobook> {
        let mut transaction = self.pool_handler.pool.begin().await?;

        let updated_active_audiobook = sqlx::query_as!(
            ActiveAudiobook,
            r#"
            UPDATE "Active_Audiobook"
            SET
                playback_position = $1,
                edited_at = current_timestamp
            WHERE user_id = $2 AND audiobook_id = $3
            RETURNING *
            "#,
            params.playback_position,
            params.user_id,
            params.audiobook_id,
        )
            .fetch_all(transaction.as_mut())
            .await?;

        if let Some(updated) = updated_active_audiobook.into_iter().nth(0) {
            transaction.commit().await?;
            return Ok(updated);
        }

        let new_active_audiobook = sqlx::query_as!(
            ActiveAudiobook,
            r#"
            INSERT INTO "Active_Audiobook" (user_id, audiobook_id, playback_position)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            params.user_id,
            params.audiobook_id,
            params.playback_position
        )
            .fetch_one(transaction.as_mut())
            .await?;

        transaction.commit().await?;

        Ok(new_active_audiobook)
    }

    /// Returns most currently listened users book
    pub async fn get_latest_active_audiobook(
        &self,
        user_id: &Id,
    ) -> DbResultSingle<Option<PlayedAudiobook>> {
        let last_active_book = sqlx::query_as!(
            PlayedAudiobookDb,
            r#"
            SELECT A.id as book_id, A.file_path AS path, A.thumbnail as thumbnail,
                A.name AS name, ACT.playback_position AS playback_position,
                B.edited_at IS NOT NULL AS is_liked, U.id as author_id,
                U.name AS author_name, U.surname As author_surname
            FROM "Active_Audiobook" ACT
            LEFT JOIN "Audiobook" A ON
                ACT.audiobook_id = A.id
            LEFT JOIN "User" U ON
                A.author_id = U.id
            LEFT JOIN "Bookmark" B ON
                A.id = B.audiobook_id
            WHERE
                ACT.user_id = $1
            ORDER BY ACT.edited_at DESC
            LIMIT 1
            "#,
            user_id,
        )
            .fetch_optional(&self.pool_handler.pool)
            .await?;
        Ok(last_active_book.map(PlayedAudiobook::from))
    }

    /// TODO: refactor this
    pub async fn get_or_create_active_audiobook(
        &self,
        user_id: &Id,
        book_id: &Id,
    ) -> DbResultSingle<PlayedAudiobook> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let exists = sqlx::query_as!(
            ActiveAudiobook,
            r#"
            SELECT *
            FROM "Active_Audiobook"
            WHERE user_id = $1 AND audiobook_id = $2
            "#,
            user_id,
            book_id,
        )
            .fetch_optional(transaction.as_mut())
            .await?;

        AudiobookRepository::increment_stream_count(book_id, &mut transaction).await?;

        if let Some(_book) = exists {
            let played_audiobook = sqlx::query_as!(
                PlayedAudiobookDb,
                r#"
                SELECT A.id as book_id, A.file_path AS path, A.thumbnail as thumbnail,
                    A.name AS name, ACT.playback_position AS playback_position,
                    B.edited_at IS NOT NULL AS is_liked, U.id as author_id,
                    U.name AS author_name, U.surname As author_surname
                FROM "Active_Audiobook" ACT
                    LEFT JOIN "Audiobook" A ON ACT.audiobook_id = A.id
                    LEFT JOIN "User" U ON A.author_id = U.id
                    LEFT JOIN "Bookmark" B ON A.id = B.audiobook_id
                WHERE ACT.user_id = $1 AND ACT.audiobook_id = $2
                "#,
                user_id,
                book_id
            )
                .fetch_one(transaction.as_mut())
                .await?;

            transaction.commit().await?;
            return Ok(PlayedAudiobook::from(played_audiobook));
        }

        sqlx::query_as!(
            ActiveAudiobook,
            r#"
            INSERT INTO "Active_Audiobook" (user_id, audiobook_id, playback_position)
            VALUES ($1, $2, 0)
            "#,
            user_id,
            book_id,
        ).execute(transaction.as_mut()).await?;

        let played_audiobook = sqlx::query_as!(
            PlayedAudiobookDb,
            r#"
                SELECT A.id as book_id, A.file_path AS path, A.thumbnail as thumbnail,
                    A.name AS name, ACT.playback_position AS playback_position,
                    B.edited_at IS NOT NULL AS is_liked, U.id as author_id,
                    U.name AS author_name, U.surname As author_surname
                FROM "Active_Audiobook" ACT
                    LEFT JOIN "Audiobook" A ON ACT.audiobook_id = A.id
                    LEFT JOIN "User" U ON A.author_id = U.id
                    LEFT JOIN "Bookmark" B ON A.id = B.audiobook_id
                WHERE ACT.user_id = $1 AND ACT.audiobook_id = $2
                "#,
            user_id,
            book_id
        )
            .fetch_one(transaction.as_mut())
            .await?;

        transaction.commit().await?;
        Ok(PlayedAudiobook::from(played_audiobook))
    }

    pub async fn get_bookmarked(&self, user_id: &Id) -> DbResultMultiple<AudiobookDisplay> {
        let bookmarked = sqlx::query_as!(
            AudiobookDetail,
            r#"
            SELECT
                a.id,
                a.name,
                a.description,
                a.file_path,
                a.length,
                a.thumbnail,
                a.overall_rating,
                a.stream_count,
                a.like_count,
                a.created_at,
                a.edited_at,
                a.deleted_at,

                a.author_id,
                u.name AS author_name,
                u.surname,
                u.username,
                u.email,
                u.profile_picture,
                u.bio,

                a.genre_id,
                g.name AS genre_name,
                g.color AS genre_color,

                ab.playback_position AS "playback_position?",
                ab.edited_at AS "active_audiobook_edited_at?",
                b.audiobook_id IS NOT NULL AS "is_liked!"
            FROM
                "Audiobook" AS a
                    INNER JOIN
                "User" AS u ON u.id = a.author_id
                    INNER JOIN
                "Genre" AS g ON a.genre_id = g.id
                    INNER JOIN
                "Bookmark" b ON b.audiobook_id = a.id
                    LEFT JOIN
                "Active_Audiobook" AS ab ON ab.audiobook_id = a.id AND ab.user_id = $1
            WHERE
                a.deleted_at IS NULL AND b.user_id = $1
            ORDER BY b.edited_at DESC
            "#,
            user_id,
        )
            .fetch_all(&self.pool_handler.pool)
            .await?;

        Ok(bookmarked.into_iter().map(AudiobookDisplay::from).collect())
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
    async fn read_one(&self, params: &AudiobookGetById) -> DbResultSingle<Audiobook> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let audiobook = AudiobookRepository::get_audiobook(
            params, &mut transaction)
            .await?;
        Ok(audiobook)
    }
}

#[async_trait]
impl DbReadOne<AudiobookGetByIdJoin, AudiobookDetail> for AudiobookRepository {
    async fn read_one(&self, params: &AudiobookGetByIdJoin) -> DbResultSingle<AudiobookDetail> {
        let maybe_audiobook = sqlx::query_as!(
            AudiobookDetail,
            r#"
            SELECT
                a.id,
                a.name,
                a.description,
                a.file_path,
                a.length,
                a.thumbnail,
                a.overall_rating,
                a.stream_count,
                a.like_count,
                a.created_at,
                a.edited_at,
                a.deleted_at,

                a.author_id,
                u.name AS author_name,
                u.surname,
                u.username,
                u.email,
                u.profile_picture,
                u.bio,

                a.genre_id,
                g.name AS genre_name,
                g.color AS genre_color,

                ab.playback_position AS "playback_position?",
                ab.edited_at AS "active_audiobook_edited_at?",
                b.audiobook_id IS NOT NULL AS "is_liked!"
            FROM
                "Audiobook" AS a
                    INNER JOIN
                "User" AS u ON u.id = a.author_id
                    INNER JOIN
                "Genre" AS g ON a.genre_id = g.id
                    LEFT JOIN
                "Active_Audiobook" AS ab ON ab.audiobook_id = a.id AND ab.user_id = $2
                    LEFT JOIN
                "Bookmark" as b ON a.id = b.audiobook_id AND b.user_id = $2
            WHERE
                a.id = $1
            "#,
            params.audiobook_id,
            params.user_id
        )
            .fetch_optional(&self.pool_handler.pool)
            .await?;

        let audiobook = entity_is_correct(maybe_audiobook, EntityError::new(AudiobookDeleted, AudiobookDoesNotExist))?;
        Ok(audiobook)
    }
}

#[async_trait]
impl DbReadMany<AudiobookSearch, AudiobookDisplay> for AudiobookRepository {
    async fn read_many(&self, params: &AudiobookSearch) -> DbResultMultiple<AudiobookDisplay> {
        let mut query = r#"
            SELECT
                a.id,
                a.name,
                a.description,
                a.file_path,
                a.length,
                a.thumbnail,
                a.overall_rating,
                a.stream_count,
                a.like_count,
                a.created_at,
                a.edited_at,
                a.deleted_at,

                a.author_id,
                u.name AS author_name,
                u.surname,
                u.username,
                u.email,
                u.profile_picture,
                u.bio,

                a.genre_id,
                g.name AS genre_name,
                g.color AS genre_color,

                ab.playback_position,
                ab.edited_at AS active_audiobook_edited_at,
                b.audiobook_id IS NOT NULL AS is_liked
            FROM
                "Audiobook" AS a
                    INNER JOIN
                "User" AS u ON u.id = a.author_id
                    INNER JOIN
                "Genre" AS g ON a.genre_id = g.id
                    LEFT JOIN
                "Active_Audiobook" AS ab ON ab.audiobook_id = a.id AND ab.user_id = $12
                    LEFT JOIN
                "Bookmark" as b ON a.id = b.audiobook_id AND b.user_id = $12
            WHERE
                a.deleted_at IS NULL
                AND u.deleted_at IS NULL
                AND g.deleted_at IS NULL
                AND (a.name = $1 OR $1 IS NULL)
                AND (author_id = $2 OR $2 IS NULL)
                AND (genre_id = $3 OR $3 IS NULL)
                AND (like_count >= $4 OR $4 IS NULL)
                AND (like_count <= $5 OR $5 IS NULL)
                AND (stream_count >= $6 OR $6 IS NULL)
                AND (stream_count <= $7 OR $7 IS NULL)
                AND (overall_rating >= $8 OR $8 IS NULL)
                AND (overall_rating <= $9 OR $9 IS NULL)
                AND (u.name = $10 OR $10 IS NULL)
                AND (g.name = $11 OR $11 IS NULL)
            "#
            .to_owned();

        let query_params = generate_query_param_string(&params.query_params);
        query.push_str(query_params.as_str());
        let audiobooks = sqlx::query_as::<_, AudiobookDetail>(query.as_str())
            .bind(&params.name)
            .bind(params.author_id)
            .bind(params.genre_id)
            .bind(params.min_like_count)
            .bind(params.max_like_count)
            .bind(params.min_stream_count)
            .bind(params.max_stream_count)
            .bind(params.min_overall_rating)
            .bind(params.max_overall_rating)
            .bind(&params.author_name)
            .bind(&params.genre_name)
            .bind(params.user_id)
            .fetch_all(&self.pool_handler.pool)
            .await?;
        Ok(audiobooks.into_iter().map(AudiobookDisplay::from).collect())
    }
}


#[async_trait]
impl DbCreate<AudiobookCreate, Audiobook> for AudiobookRepository {
    async fn create(&self, params: &AudiobookCreate) -> DbResultSingle<Audiobook> {
        let book = sqlx::query_as!(
            Audiobook,
            r#"
            INSERT INTO "Audiobook" (name, author_id, genre_id, file_path, length, thumbnail, description)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            params.name,
            params.author_id,
            params.genre_id,
            params.file_path,
            params.length,
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
            &AudiobookGetById { id: params.id },
            &mut transaction,
        )
            .await?;
        let updated_audio_books = sqlx::query_as!(
            Audiobook,
            r#"
            UPDATE "Audiobook"
            SET
                name = COALESCE($1, name),
                author_id = COALESCE($2, author_id),
                genre_id = COALESCE($3, genre_id),
                file_path = COALESCE($4, file_path),
                length = COALESCE($5, length),
                stream_count = COALESCE($6, stream_count),
                like_count = COALESCE($7, like_count),
                overall_rating = COALESCE($8, overall_rating),
                thumbnail = COALESCE($9, thumbnail),
                description = COALESCE($10, description),
                edited_at = current_timestamp
            WHERE id = $11
            RETURNING *
            "#,
            params.name,
            params.author_id,
            params.genre_id,
            params.file_path,
            params.length,
            params.stream_count,
            params.like_count,
            params.overall_rating,
            params.thumbnail,
            params.description,
            audiobook.id
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
        let audiobook = AudiobookRepository::get_audiobook(
            &AudiobookGetById { id: params.id },
            &mut transaction,
        )
            .await?;

        sqlx::query!(
            r#"
            DELETE FROM "Bookmark"
            WHERE audiobook_id = $1
            "#,
            audiobook.id,
        )
            .fetch_all(transaction.as_mut())
            .await?;

        sqlx::query!(
            r#"
            DELETE FROM "Active_Audiobook"
            WHERE audiobook_id = $1
            "#,
            audiobook.id,
        )
            .fetch_all(transaction.as_mut())
            .await?;

        sqlx::query!(
            r#"
            UPDATE "Chapter" SET
                deleted_at = current_timestamp,
                edited_at = current_timestamp
            WHERE audiobook_id = $1
            "#,
            audiobook.id,
        )
            .fetch_all(transaction.as_mut())
            .await?;

        let books = sqlx::query_as!(
            Audiobook,
            r#"
            UPDATE "Audiobook" SET
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
