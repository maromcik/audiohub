use crate::database::common::error::BackendErrorKind::{
    ChapterDeleted, ChapterDoesNotExist, RatingUpdateParametersEmpty,
};
use crate::database::common::error::{BackendError, DbError, DbResultMultiple, DbResultSingle};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler,
};
use crate::database::models::audiobook::AudiobookGetById;
use crate::database::models::chapter::{
    Chapter, ChapterCreate, ChapterGetByBookId, ChapterGetById, ChapterSearch, ChapterUpdate,
};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

#[derive(Clone)]
pub struct ChapterRepository {
    pool_handler: PoolHandler,
}

impl ChapterRepository {
    /// Function which retrieves chapter by id, usable within a transaction
    ///
    /// # Params
    /// - `params`: structure containing the id of the chapter
    /// - `transaction_handle` mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - `Ok(chapter)`: on successful connection and retrieval
    /// - `Err(_)`: otherwise
    pub async fn get<'a>(
        params: &ChapterGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Chapter>> {
        let query = sqlx::query_as!(
            Chapter,
            r#"
            SELECT * FROM "Chapter"
            WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        if let Some(chapter) = query {
            return Ok(Some(chapter));
        }

        Err(DbError::from(BackendError::new(ChapterDoesNotExist)))
    }

    /// Function which retrieves all chapters for book with given id, usable within a transaction
    ///
    /// # Params
    /// - `params`: structure containing the id of the book
    /// - `transaction_handle` mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - `Ok(chapters)`: on successful connection and retrieval
    /// - `Err(_)`: otherwise
    pub async fn get_book_chapters<'a>(
        params: AudiobookGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultMultiple<Chapter> {
        let chapters = sqlx::query_as!(
            Chapter,
            r#"
                SELECT * FROM "Chapter"
                WHERE audiobook_id = $1
                "#,
            params.id
        )
        .fetch_all(transaction_handle.as_mut())
        .await?;

        Ok(chapters)
    }

    pub async fn delete_chapter<'a>(
        params: &ChapterGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Chapter> {
        let chapter = sqlx::query_as!(
            Chapter,
            r#"
            UPDATE "Chapter"
            SET
                deleted_at = current_timestamp,
                edited_at = current_timestamp
            WHERE id = $1
            RETURNING *"#,
            params.id
        )
        .fetch_one(transaction_handle.as_mut())
        .await?;

        Ok(chapter)
    }

    pub async fn update<'a>(
        params: &ChapterUpdate,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Chapter> {
        let chapter = sqlx::query_as!(
            Chapter,
            r#"
            UPDATE "Chapter"
            SET
                name = COALESCE($1, name),
                edited_at = current_timestamp
            WHERE id = $2
            RETURNING *
            "#,
            params.name,
            params.id
        )
        .fetch_one(transaction_handle.as_mut())
        .await?;

        return Ok(chapter);
    }

    /// Function which checks if the chapter is correct (existing and not deleted)
    ///
    /// # Params
    /// - `user`: optional chapter retrieved from the database
    ///
    /// # Returns
    /// - `Ok(chapter)`: when the chapter exists and is not deleted
    /// - `Err(DbError)`: with appropriate error description otherwise
    pub fn is_correct(chapter: Option<Chapter>) -> DbResultSingle<Chapter> {
        if let Some(chapter) = chapter {
            if chapter.deleted_at.is_none() {
                return Ok(chapter);
            }
            return Err(DbError::from(BackendError::new(ChapterDeleted)));
        }

        Err(DbError::from(BackendError::new(ChapterDoesNotExist)))
    }
}

#[async_trait]
impl DbRepository for ChapterRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    #[inline]
    async fn disconnect(&self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbCreate<ChapterCreate, Chapter> for ChapterRepository {
    async fn create(&self, params: &ChapterCreate) -> DbResultSingle<Chapter> {
        let chapter = sqlx::query_as!(
            Chapter,
            r#"
            INSERT INTO "Chapter" (name, audiobook_id, length, sequential_number)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            params.name,
            params.audiobook_id,
            params.length,
            params.sequential_number,
        )
        .fetch_one(&self.pool_handler.pool)
        .await?;

        Ok(chapter)
    }
}

#[async_trait]
impl DbReadOne<ChapterGetById, Chapter> for ChapterRepository {
    async fn read_one(&self, params: &ChapterGetById) -> DbResultSingle<Chapter> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let chapter = ChapterRepository::get(params, &mut transaction).await?;
        let chapter = ChapterRepository::is_correct(chapter);
        transaction.commit().await?;
        chapter
    }
}

#[async_trait]
impl DbReadMany<ChapterSearch, Chapter> for ChapterRepository {
    async fn read_many(&self, params: &ChapterSearch) -> DbResultMultiple<Chapter> {
        let chapters = sqlx::query_as!(
            Chapter,
            r#"
            SELECT * FROM "Chapter"
            WHERE
                (name = $1 OR $1 IS NULL)
                AND (audiobook_id = $2 OR $2 IS NULL)
                AND (sequential_number = $3 OR $3 IS NULL)
            "#,
            params.name,
            params.audiobook_id,
            params.sequential_number,
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(chapters)
    }
}

#[async_trait]
impl DbReadMany<ChapterGetByBookId, Chapter> for ChapterRepository {
    async fn read_many(&self, params: &ChapterGetByBookId) -> DbResultMultiple<Chapter> {
        let chapters = sqlx::query_as!(
            Chapter,
            r#"
            SELECT * FROM "Chapter"
            WHERE audiobook_id = $1
            "#,
            params.audiobook_id,
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(chapters)
    }
}

#[async_trait]
impl DbDelete<ChapterGetById, Chapter> for ChapterRepository {
    async fn delete(&self, params: &ChapterGetById) -> DbResultMultiple<Chapter> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let chapter = ChapterRepository::get(params, &mut transaction).await?;
        ChapterRepository::is_correct(chapter)?;

        let chapter = ChapterRepository::delete_chapter(params, &mut transaction).await?;
        transaction.commit().await?;

        Ok(vec![chapter])
    }
}

#[async_trait]
impl DbUpdate<ChapterUpdate, Chapter> for ChapterRepository {
    async fn update(&self, params: &ChapterUpdate) -> DbResultMultiple<Chapter> {
        if params.name.is_none() {
            return Err(DbError::from(BackendError::new(
                RatingUpdateParametersEmpty,
            )));
        }

        let mut transcation = self.pool_handler.pool.begin().await?;

        let chapter =
            ChapterRepository::get(&ChapterGetById { id: params.id }, &mut transcation).await?;
        ChapterRepository::is_correct(chapter)?;

        let chapter = ChapterRepository::update(params, &mut transcation).await?;

        transcation.commit().await?;
        Ok(vec![chapter])
    }
}
