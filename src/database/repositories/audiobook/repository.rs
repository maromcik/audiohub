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
use chrono::{Duration, Utc};
use sqlx::{Acquire, Postgres, QueryBuilder, Transaction};

use crate::database::models::audiobook::{
    Audiobook, AudiobookCreate, AudiobookDelete, AudiobookGetById, AudiobookUpdate,
};
use crate::database::models::Id;

pub struct AudiobookRepository {
    pool_handler: PoolHandler,
}

impl AudiobookRepository {
    pub async fn get_audiobook<'a>(
        params: AudiobookGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Audiobook>> {
        let mut tx = transaction_handle.begin().await?;

        let query = sqlx::query_as::<_, Audiobook>(r#"SELECT * FROM "Audiobook" WHERE id = $1"#)
            .bind(params.id)
            .fetch_optional(&mut *tx)
            .await?;

        if let Some(book) = query {
            tx.commit().await?;
            return Ok(Option::from(book));
        }

        Err(DbError::from(BusinessLogicError::new(
            AudiobookDoesNotExist,
        )))
    }

    pub fn build_query(update_info: &AudiobookUpdate) -> QueryBuilder<Postgres> {
        // let change_to_option_i64 = |value| Some(value.into());

        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new(r#"UPDATE "Audiobook" SET "#);

        AudiobookRepository::add_string_to_query(&mut query_builder, &update_info.name, "name");
        AudiobookRepository::add_id_to_query(
            &mut query_builder,
            &update_info.publisher_id,
            "publisher_id",
        );
        AudiobookRepository::add_id_to_query(&mut query_builder, &update_info.genre_id, "genre_id");
        AudiobookRepository::add_id_to_query(
            &mut query_builder,
            &update_info.author_id,
            "author_id",
        );
        AudiobookRepository::add_id_to_query(
            &mut query_builder,
            &update_info.price_dollars.map(|x| x as i64),
            "price_dollars",
        );
        AudiobookRepository::add_id_to_query(
            &mut query_builder,
            &update_info.price_cents.map(|x| x as i64),
            "price_cents",
        );
        AudiobookRepository::add_length_to_query(&mut query_builder, &update_info.length, "length");
        AudiobookRepository::add_string_to_query(
            &mut query_builder,
            &update_info.file_path,
            "file_path",
        );
        AudiobookRepository::add_id_to_query(
            &mut query_builder,
            &update_info.stream_count,
            "stream_count",
        );
        AudiobookRepository::add_id_to_query(
            &mut query_builder,
            &update_info.overall_rating.map(|x| x as i64),
            "overall_rating",
        );

        let time_of_edit = Utc::now();
        query_builder.push(format!("edited_at = '{}' ", time_of_edit));
        query_builder.push(format!(
            " WHERE id = '{}'\
        RETURNING *",
            update_info.id
        ));

        query_builder
    }

    fn add_string_to_query(
        query_builder: &mut QueryBuilder<Postgres>,
        string_value: &Option<String>,
        name: &str,
    ) {
        if let Some(val) = &string_value {
            query_builder.push(format!("{} = '{}', ", name, val));
        }
    }

    fn add_id_to_query(
        query_builder: &mut QueryBuilder<Postgres>,
        id_value: &Option<Id>,
        name: &str,
    ) {
        if let Some(val) = &id_value {
            query_builder.push(format!("{} = '{}', ", name, val));
        }
    }

    fn add_length_to_query(
        query_builder: &mut QueryBuilder<Postgres>,
        string_value: &Option<Duration>,
        name: &str,
    ) {
        if let Some(val) = &string_value {
            query_builder.push(format!("{} = '{}', ", name, val));
        }
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
    async fn create(&mut self, data: &AudiobookCreate) -> DbResultSingle<Audiobook> {
        let created_at = Utc::now();
        let book = sqlx::query_as::<_, Audiobook>(
            r#"INSERT INTO "Audiobook" (name, author_id, publisher_id, genre_id,
            price_dollars, price_cents, length, file_path, stream_count,
            overall_rating, created_at, edited_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *"#,
        )
        .bind(&data.name)
        .bind(data.author_id)
        .bind(data.publisher_id)
        .bind(data.genre_id)
        .bind(data.price_dollars)
        .bind(data.price_cents)
        .bind(data.length)
        .bind(&data.file_path)
        .bind(data.stream_count)
        .bind(data.overall_rating)
        .bind(created_at)
        .bind(created_at)
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

        let book_query = AudiobookRepository::get_audiobook(
            AudiobookGetById { id: params.id },
            &mut transaction,
        )
        .await?;
        let _ = AudiobookRepository::audiobook_is_correct(book_query.clone())?;

        let mut query_builder = AudiobookRepository::build_query(params);
        let books = query_builder
            .build_query_as()
            .fetch_all(transaction.as_mut())
            .await?;
        Ok(books)
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

        let id = params.id;
        let deleted_at = Utc::now();

        let books = sqlx::query_as::<_, Audiobook>(
            r#"
                UPDATE "Audiobook" SET
                     name = $1,
                     deleted_at = $2,
                     edited_at = $2
                WHERE id = $1
                RETURNING *
            "#,
        )
        .bind(id)
        .bind(deleted_at)
        .fetch_all(transaction.as_mut())
        .await?;
        transaction.commit().await?;

        Ok(books)
    }
}
