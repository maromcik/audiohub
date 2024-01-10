use crate::database::common::error::BusinessLogicErrorKind::{
    PublisherDeleted, PublisherDoesNotExist, PublisherUpdateParametersEmpty,
};
use crate::database::common::error::{
    BusinessLogicError, DbError, DbResultMultiple, DbResultSingle,
};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler,
};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::database::models::publisher::{
    Publisher, PublisherCreate, PublisherDelete, PublisherGetById, PublisherSearch, PublisherUpdate,
};

#[derive(Clone)]
pub struct PublisherRepository {
    pool_handler: PoolHandler,
}

impl PublisherRepository {
    pub async fn get_publisher<'a>(
        params: PublisherGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Publisher>> {
        let query = sqlx::query_as!(
            Publisher,
            r#"
            SELECT * FROM "Publisher"
            WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        if let Some(publisher) = query {
            return Ok(Some(publisher));
        }

        Err(DbError::from(BusinessLogicError::new(
            PublisherDoesNotExist,
        )))
    }

    pub fn publisher_is_correct(publisher: Option<Publisher>) -> DbResultSingle<Publisher> {
        if let Some(publisher) = publisher {
            if publisher.deleted_at.is_none() {
                return Ok(publisher);
            }
            return Err(DbError::from(BusinessLogicError::new(PublisherDeleted)));
        }

        Err(DbError::from(BusinessLogicError::new(
            PublisherDoesNotExist,
        )))
    }
}

#[async_trait]
impl DbRepository for PublisherRepository {
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
impl DbReadOne<PublisherGetById, Publisher> for PublisherRepository {
    /// Login the user with provided parameters, if the user does not exist, is deleted or the
    /// passwords don't match, return the error about combination of email/password not working
    async fn read_one(&mut self, params: &PublisherGetById) -> DbResultSingle<Publisher> {
        let maybe_publisher = sqlx::query_as!(
            Publisher,
            r#"
            SELECT * FROM "Publisher"
            WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(&*self.pool_handler.pool)
        .await?;

        let publisher = PublisherRepository::publisher_is_correct(maybe_publisher)?;
        Ok(publisher)
    }
}

#[async_trait]
impl DbReadMany<PublisherSearch, Publisher> for PublisherRepository {
    async fn read_many(&mut self, params: &PublisherSearch) -> DbResultMultiple<Publisher> {
        let publishers = sqlx::query_as!(
            Publisher,
            r#"
            SELECT * FROM "Publisher"
            WHERE
                (name = $1 OR $1 IS NULL)
            "#,
            params.name
        )
        .fetch_all(self.pool_handler.pool.as_ref())
        .await?;
        Ok(publishers)
    }
}

#[async_trait]
impl DbCreate<PublisherCreate, Publisher> for PublisherRepository {
    /// Create a new publisher with the given data
    async fn create(&mut self, params: &PublisherCreate) -> DbResultSingle<Publisher> {
        let publisher = sqlx::query_as!(
            Publisher,
            r#"
            INSERT INTO "Publisher" (name)
            VALUES ($1)
            RETURNING *
            "#,
            params.name
        )
        .fetch_one(&*self.pool_handler.pool)
        .await?;

        Ok(publisher)
    }
}

#[async_trait]
impl DbUpdate<PublisherUpdate, Publisher> for PublisherRepository {
    async fn update(&mut self, params: &PublisherUpdate) -> DbResultMultiple<Publisher> {
        if params.update_fields_none() {
            return Err(DbError::from(BusinessLogicError::new(
                PublisherUpdateParametersEmpty,
            )));
        }

        let mut transaction = self.pool_handler.pool.begin().await?;
        let publisher_id = PublisherGetById::new(&params.id);

        let query_publisher =
            PublisherRepository::get_publisher(publisher_id, &mut transaction).await?;
        let _ = PublisherRepository::publisher_is_correct(query_publisher);

        let publishers = sqlx::query_as!(
            Publisher,
            r#"
            UPDATE "Publisher"
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
        Ok(publishers)
    }
}

#[async_trait]
impl DbDelete<PublisherDelete, Publisher> for PublisherRepository {
    async fn delete(&mut self, params: &PublisherDelete) -> DbResultMultiple<Publisher> {
        let mut transaction = self.pool_handler.pool.begin().await?;

        // Check existence
        let _ = PublisherRepository::get_publisher(
            PublisherGetById { id: params.id },
            &mut transaction,
        )
        .await?;

        let publishers = sqlx::query_as!(
            Publisher,
            r#"
                UPDATE "Publisher" SET
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

        Ok(publishers)
    }
}
