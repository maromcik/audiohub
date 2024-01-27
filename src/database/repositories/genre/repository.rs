use crate::database::common::error::BackendErrorKind::{GenreDeleted, GenreDoesNotExist, GenreUpdateParametersEmpty};
use crate::database::common::error::{BackendError, DbError, DbResultMultiple, DbResultSingle, EntityError};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler,
};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use crate::database::common::utilities::entity_is_correct;

use crate::database::models::genre::{
    Genre, GenreCreate, GenreDelete, GenreGetById, GenreSearch, GenreUpdate,
};

#[derive(Clone)]
pub struct GenreRepository {
    pool_handler: PoolHandler,
}

impl GenreRepository {
    pub async fn get_genre<'a>(
        params: GenreGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Genre>> {
        let query = sqlx::query_as!(
            Genre,
            r#"
            SELECT * FROM "Genre"
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        if let Some(genre) = query {
            return Ok(Some(genre));
        }

        Err(DbError::from(BackendError::new(GenreDoesNotExist)))
    }

    pub fn genre_is_correct(genre: Option<Genre>) -> DbResultSingle<Genre> {
        entity_is_correct(genre, EntityError::new(GenreDeleted, GenreDoesNotExist), false)
    }
}

#[async_trait]
impl DbRepository for GenreRepository {
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
impl DbReadOne<GenreGetById, Genre> for GenreRepository {
    /// Login the user with provided parameters, if the user does not exist, is deleted or the
    /// passwords don't match, return the error about combination of email/password not working
    async fn read_one(&self, params: &GenreGetById) -> DbResultSingle<Genre> {
        let maybe_genre = sqlx::query_as!(
            Genre,
            r#"
            SELECT * FROM "Genre"
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            params.id
        )
        .fetch_optional(&self.pool_handler.pool)
        .await?;

        let genre = GenreRepository::genre_is_correct(maybe_genre)?;
        Ok(genre)
    }
}

#[async_trait]
impl DbReadMany<GenreSearch, Genre> for GenreRepository {
    async fn read_many(&self, params: &GenreSearch) -> DbResultMultiple<Genre> {
        let genres = sqlx::query_as!(
            Genre,
            r#"
            SELECT * FROM "Genre"
            WHERE
                (name = $1 OR $1 IS NULL)
                 AND deleted_at IS NULL
            ORDER BY name"#,
            params.name
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(genres)
    }
}

#[async_trait]
impl DbCreate<GenreCreate, Genre> for GenreRepository {
    /// Create a new genre with the given data
    async fn create(&self, params: &GenreCreate) -> DbResultSingle<Genre> {
        let genre = sqlx::query_as!(
            Genre,
            r#"
            INSERT INTO "Genre" (name)
            VALUES ($1)
            RETURNING *
            "#,
            params.name,
        )
        .fetch_one(&self.pool_handler.pool)
        .await?;

        Ok(genre)
    }
}

#[async_trait]
impl DbUpdate<GenreUpdate, Genre> for GenreRepository {
    async fn update(&self, params: &GenreUpdate) -> DbResultMultiple<Genre> {
        if params.update_fields_none() {
            return Err(DbError::from(BackendError::new(GenreUpdateParametersEmpty)));
        }

        let mut transaction = self.pool_handler.pool.begin().await?;
        let genre_id = GenreGetById::new(&params.id);

        let query_genre = GenreRepository::get_genre(genre_id, &mut transaction).await?;
        let _ = GenreRepository::genre_is_correct(query_genre);

        let genres = sqlx::query_as!(
            Genre,
            r#"
            UPDATE "Genre"
            SET
                name = COALESCE($1, name),
                color = COALESCE($2, color),
                edited_at = current_timestamp
            WHERE id = $3
            RETURNING *
            "#,
            params.name,
            params.color,
            params.id
        )
        .fetch_all(transaction.as_mut())
        .await?;

        transaction.commit().await?;
        Ok(genres)
    }
}

#[async_trait]
impl DbDelete<GenreDelete, Genre> for GenreRepository {
    async fn delete(&self, params: &GenreDelete) -> DbResultMultiple<Genre> {
        let mut transaction = self.pool_handler.pool.begin().await?;

        // Check existence
        let _ =
            GenreRepository::get_genre(GenreGetById { id: params.id }, &mut transaction).await?;

        let genres = sqlx::query_as!(
            Genre,
            r#"
                UPDATE "Genre" SET
                    name = $1,
                    deleted_at = current_timestamp,
                    edited_at = current_timestamp
                WHERE id = $1
                RETURNING *
               "#,
            params.id
        )
        .fetch_all(transaction.as_mut())
        .await?;

        transaction.commit().await?;

        Ok(genres)
    }
}
