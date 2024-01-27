use crate::database::common::error::BackendErrorKind::{ChapterDeleted, ChapterDoesNotExist, ChapterUpdateParametersEmpty};
use crate::database::common::error::{BackendError, DbError, DbResultMultiple, DbResultSingle, EntityError};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler,
};

use crate::database::models::chapter::{Chapter, ChapterCreate, ChapterGetById, ChapterSearch, ChapterUpdate, ChaptersGetByBookId, ChapterDetail, ChaptersGetByBookIdJoin};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use crate::database::common::utilities::entity_is_correct;

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
            WHERE id = $1 AND deleted_at IS NULL
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

    /// Function which retrieves all chapters in displayable form for book with given id
    ///
    /// # Params
    /// - `params`: structure containing the id of the book
    /// - `transaction_handle` mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - `Ok(chapters)`: on successful connection and retrieval
    /// - `Err(_)`: otherwise
    pub async fn delete_chapter<'a>(
        params: &ChapterGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Chapter> {
        let chapter = sqlx::query_as!(
            Chapter,
            r#"
            DELETE FROM "Chapter"
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
        entity_is_correct(chapter, EntityError::new(ChapterDeleted, ChapterDoesNotExist), false)
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
            INSERT INTO "Chapter" (name, audiobook_id, position)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            params.name,
            params.audiobook_id,
            params.position
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
                AND deleted_at IS NULL
            "#,
            params.name,
            params.audiobook_id
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(chapters)
    }
}

#[async_trait]
impl DbReadMany<ChaptersGetByBookId, Chapter> for ChapterRepository {
    async fn read_many(&self, params: &ChaptersGetByBookId) -> DbResultMultiple<Chapter> {
        let chapters = sqlx::query_as!(
            Chapter,
            r#"
            SELECT
                *
            FROM
                "Chapter"
            WHERE
                deleted_at IS NULL
                AND audiobook_id = $1
            ORDER BY
                position
            "#,
            params.audiobook_id,
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(chapters)
    }
}

#[async_trait]
impl DbReadMany<ChaptersGetByBookIdJoin, ChapterDetail> for ChapterRepository {
    async fn read_many(&self, params: &ChaptersGetByBookIdJoin) -> DbResultMultiple<ChapterDetail> {
        let chapters = sqlx::query_as!(
            ChapterDetail,
            r#"
            SELECT
                c.id,
                c.name,
                c.audiobook_id,
                c.position,
                c.created_at,
                c.edited_at,
                c.deleted_at,
                a.name AS audiobook_name,
                a.author_id
            FROM
                "Chapter" AS c
                    INNER JOIN
                "Audiobook" AS a ON c.audiobook_id = a.id
            WHERE
                c.deleted_at IS NULL
                AND c.audiobook_id = $1
            ORDER BY
                c.position
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
        let chapter = ChapterRepository::delete_chapter(&ChapterGetById::new(params.id), &mut transaction).await?;
        transaction.commit().await?;
        Ok(vec![chapter])
    }
}

#[async_trait]
impl DbUpdate<ChapterUpdate, Chapter> for ChapterRepository {
    async fn update(&self, params: &ChapterUpdate) -> DbResultMultiple<Chapter> {
        if params.name.is_none() {
            return Err(DbError::from(BackendError::new(
                ChapterUpdateParametersEmpty,
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
