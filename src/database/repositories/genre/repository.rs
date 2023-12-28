use crate::database::common::error::BusinessLogicErrorKind::{
    GenreDeleted, GenreDoesNotExist, GenreUpdateParametersEmpty,
};
use crate::database::common::error::{
    BusinessLogicError, DbError, DbResultMultiple, DbResultSingle,
};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbRepository, DbUpdate, PoolHandler,
};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::database::models::genre::{Genre, GenreCreate, GenreDelete, GenreGetById, GenreUpdate};

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
            WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        if let Some(genre) = query {
            return Ok(Some(genre));
        }

        Err(DbError::from(BusinessLogicError::new(GenreDoesNotExist)))
    }

    pub fn genre_is_correct(genre: Option<Genre>) -> DbResultSingle<Genre> {
        if let Some(genre) = genre {
            if genre.deleted_at.is_none() {
                return Ok(genre);
            }
            return Err(DbError::from(BusinessLogicError::new(GenreDeleted)));
        }

        Err(DbError::from(BusinessLogicError::new(GenreDoesNotExist)))
    }
}

#[async_trait]
impl DbRepository for GenreRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    #[inline]
    async fn disconnect(&mut self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbCreate<GenreCreate, Genre> for GenreRepository {
    /// Create a new genre with the given data
    async fn create(&mut self, params: &GenreCreate) -> DbResultSingle<Genre> {
        let genre = sqlx::query_as!(
            Genre,
            r#"
            INSERT INTO "Genre" (name)
            VALUES ($1)
            RETURNING *
            "#,
            params.name
        )
        .fetch_one(&*self.pool_handler.pool)
        .await?;

        Ok(genre)
    }
}

#[async_trait]
impl DbUpdate<GenreUpdate, Genre> for GenreRepository {
    async fn update(&mut self, params: &GenreUpdate) -> DbResultMultiple<Genre> {
        if params.update_fields_none() {
            return Err(DbError::from(BusinessLogicError::new(
                GenreUpdateParametersEmpty,
            )));
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
                edited_at = current_timestamp
            WHERE id = $2
            RETURNING *
            "#,
            params.name,
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
    async fn delete(&mut self, params: &GenreDelete) -> DbResultMultiple<Genre> {
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
