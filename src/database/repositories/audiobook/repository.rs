use crate::database::common::error::BusinessLogicErrorKind::{
    AudiobookDeleted, AudiobookDoesNotExist, AudiobookUpdateParametersEmpty,
};
use crate::database::common::error::{
    BusinessLogicError, DbError, DbResultMultiple, DbResultSingle,
};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbRepository, DbUpdate, PoolHandler,
};
use async_trait::async_trait;

use sqlx::{Postgres, Transaction};

use crate::database::models::audiobook::{
    Audiobook, AudiobookCreate, AudiobookDelete, AudiobookGetById, AudiobookUpdate,
};

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

        Err(DbError::from(BusinessLogicError::new(
            AudiobookDoesNotExist,
        )))
    }

    pub fn audiobook_is_correct(audiobook: Option<Audiobook>) -> DbResultSingle<Audiobook> {
        if let Some(audiobook) = audiobook {
            if audiobook.deleted_at.is_none() {
                return Ok(audiobook);
            }
            return Err(DbError::from(BusinessLogicError::new(AudiobookDeleted)));
        }

        Err(DbError::from(BusinessLogicError::new(
            AudiobookDoesNotExist,
        )))
    }
}

#[async_trait]
impl DbRepository for AudiobookRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    async fn disconnect(&mut self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbCreate<AudiobookCreate, Audiobook> for AudiobookRepository {
    async fn create(&mut self, params: &AudiobookCreate) -> DbResultSingle<Audiobook> {
        let book = sqlx::query_as!(
            Audiobook,
            r#"
            INSERT INTO "Audiobook" (name, author_id, publisher_id, genre_id, price_dollars, price_cents, length, file_path, stream_count, overall_rating)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
            params.name,
            params.author_id,
            params.publisher_id,
            params.genre_id,
            params.price_dollars,
            params.price_cents,
            params.length,
            params.file_path,
            params.stream_count,
            params.overall_rating
        )
        .fetch_one(&*self.pool_handler.pool)
        .await?;

        Ok(book)
    }
}

#[async_trait]
impl DbUpdate<AudiobookUpdate, Audiobook> for AudiobookRepository {
    async fn update(&mut self, params: &AudiobookUpdate) -> DbResultMultiple<Audiobook> {
        if params.update_fields_none() {
            return Err(DbError::from(BusinessLogicError::new(
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
                publisher_id = COALESCE($3, publisher_id),
                genre_id = COALESCE($4, genre_id),
                price_dollars = COALESCE($5, price_dollars),
                price_cents = COALESCE($6, price_cents),
                length = COALESCE($7, length),
                file_path = COALESCE($8, file_path),
                stream_count = COALESCE($9, stream_count),
                overall_rating = COALESCE($10, overall_rating),
                edited_at = current_timestamp
            WHERE id = $11
            RETURNING *
            "#,
            params.name,
            params.author_id,
            params.publisher_id,
            params.genre_id,
            params.price_dollars,
            params.price_cents,
            params.length,
            params.file_path,
            params.stream_count,
            params.overall_rating,
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
    async fn delete(&mut self, params: &AudiobookDelete) -> DbResultMultiple<Audiobook> {
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
