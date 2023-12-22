use crate::database::common::error::BusinessLogicErrorKind::{
    AuthorDeleted, AuthorDoesNotExist, AuthorUpdateParametersEmpty,
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

use crate::database::models::author::{
    Author, AuthorCreate, AuthorDelete, AuthorGetById, AuthorUpdate,
};

pub struct AuthorRepository {
    pool_handler: PoolHandler,
}

impl AuthorRepository {
    pub async fn get_author<'a>(
        params: AuthorGetById,
        transaction: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Author>> {
        let mut tx = transaction.begin().await?;

        let query = sqlx::query_as::<_, Author>(r#"SELECT * FROM "Author" WHERE id = $1"#)
            .bind(params.id)
            .fetch_optional(&mut *tx)
            .await?;

        if let Some(author) = query {
            tx.commit().await?;
            return Ok(Some(author));
        }

        Err(DbError::from(BusinessLogicError::new(AuthorDoesNotExist)))
    }

    pub fn author_is_correct(author: Option<Author>) -> DbResultSingle<Author> {
        if let Some(author) = author {
            if author.deleted_at.is_none() {
                return Ok(author);
            }
            return Err(DbError::from(BusinessLogicError::new(AuthorDeleted)));
        }

        Err(DbError::from(BusinessLogicError::new(AuthorDoesNotExist)))
    }
}

#[async_trait]
impl DbRepository for AuthorRepository {
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
impl DbCreate<AuthorCreate, Author> for AuthorRepository {
    /// Create a new author with the given data
    async fn create(&mut self, params: &AuthorCreate) -> DbResultSingle<Author> {
        let author = sqlx::query_as::<_, Author>(
            r#"
            INSERT INTO "Author" (name)
            VALUES ($)
            RETURNING *"#,
        )
        .bind(&params.name)
        .fetch_one(&*self.pool_handler.pool)
        .await?;

        Ok(author)
    }
}

#[async_trait]
impl DbUpdate<AuthorUpdate, Author> for AuthorRepository {
    async fn update(&mut self, params: &AuthorUpdate) -> DbResultMultiple<Author> {
        if params.update_fields_none() {
            return Err(DbError::from(BusinessLogicError::new(
                AuthorUpdateParametersEmpty,
            )));
        }

        let mut transaction = self.pool_handler.pool.begin().await?;
        let author_id = AuthorGetById::new(&params.id);

        let query_author = AuthorRepository::get_author(author_id, &mut transaction).await?;
        let _ = AuthorRepository::author_is_correct(query_author);
        let authors = sqlx::query_as!(
            Author,
            r#"
                UPDATE "Author"
                SET
                    name = COALESCE($1, name),
                    edited_at = $2
                    WHERE id = $3
                RETURNING *
            "#,
            params.name,
            Utc::now(),
            params.id
        )
        .fetch_all(transaction.as_mut())
        .await?;

        transaction.commit().await?;
        Ok(authors)
    }
}

#[async_trait]
impl DbDelete<AuthorDelete, Author> for AuthorRepository {
    async fn delete(&mut self, params: &AuthorDelete) -> DbResultMultiple<Author> {
        let mut transaction = self.pool_handler.pool.begin().await?;

        // Check existence
        let _ =
            AuthorRepository::get_author(AuthorGetById { id: params.id }, &mut transaction).await?;

        let id = params.id;
        let deleted_at = Utc::now();
        let authors = sqlx::query_as::<_, Author>(
            r#"
                UPDATE "Author" SET
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

        Ok(authors)
    }
}
