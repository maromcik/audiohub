use crate::database::common::error::BusinessLogicErrorKind::{
    PublisherDeleted, PublisherDoesNotExist, PublisherUpdateParametersEmpty,
};
use crate::database::common::error::{
    BusinessLogicError, DbError, DbResultMultiple, DbResultSingle,
};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbRepository, DbUpdate, PoolHandler,
};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Acquire, Postgres, Transaction};

use crate::database::models::publisher::{
    Publisher, PublisherCreate, PublisherDelete, PublisherGetById, PublisherUpdate,
};

pub struct PublisherRepository {
    pool_handler: PoolHandler,
}

impl PublisherRepository {
    pub async fn get_publisher<'a>(
        params: PublisherGetById,
        transaction: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Publisher>> {
        let mut tx = transaction.begin().await?;

        let query = sqlx::query_as::<_, Publisher>(r#"SELECT * FROM "Publisher" WHERE id = $1"#)
            .bind(params.id)
            .fetch_optional(&mut *tx)
            .await?;

        if let Some(publisher) = query {
            tx.commit().await?;
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
impl DbCreate<PublisherCreate, Publisher> for PublisherRepository {
    /// Create a new publisher with the given data
    async fn create(&mut self, data: &PublisherCreate) -> DbResultSingle<Publisher> {
        let created_at = Utc::now();
        let publisher = sqlx::query_as::<_, Publisher>(
            r#"INSERT INTO "Publisher" (name, created_at, edited_at)
            VALUES ($1, $2, $3)
            RETURNING *"#,
        )
        .bind(&data.name)
        .bind(created_at)
        .bind(created_at)
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

        let edited_at = Utc::now();
        let publishers = sqlx::query_as::<_, Publisher>(
            r#"UPDATE "Publisher" Set
                        name = $1
                        edited_at = $2
                   WHERE id = $3
                   RETURNING *"#,
        )
        .bind(params.name.clone())
        .bind(edited_at)
        .bind(params.id)
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

        let id = params.id;
        let deleted_at = Utc::now();
        let publishers = sqlx::query_as::<_, Publisher>(
            r#"
                UPDATE "Publisher" SET
                    name = $1,
                    delete_at = $2
                    edited_at = $2
                WHERE id = $1
                RETURNING *
               "#,
        )
        .bind(id)
        .bind(deleted_at)
        .fetch_all(transaction.as_mut())
        .await?;

        //Check audiobooks and delete them?

        transaction.commit().await?;

        Ok(publishers)
    }
}
