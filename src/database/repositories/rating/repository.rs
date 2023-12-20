use async_trait::async_trait;
use sqlx::{PgConnection, Postgres, QueryBuilder, Transaction};
use crate::database::common::{DbCreate, DbDelete, DbPoolHandler, DbReadOne, DbRepository, DbUpdate, PoolHandler};
use crate::database::common::error::{BusinessLogicError, DbError, DbResultMultiple, DbResultSingle};
use crate::database::common::error::BusinessLogicErrorKind::{RatingDeleted, RatingDoesNotExist, RatingUpdateEmpty};
use crate::database::models::rating::{Rating, RatingCreate, RatingGetById, RatingUpdate};
use crate::database::models::user::{UserGetById};
use crate::database::models::audiobook::{AudiobookGetById};


pub struct RatingRepository {
    pool_handler: PoolHandler,
}

impl RatingRepository {

    /// Function which retrieves rating by  id, usable within a transaction
    ///
    /// # Params
    /// - `params`: structure containing the id of the rating
    /// - `transaction_handle` mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - `Ok(rating)`: on successful connection and retrieval
    /// - `Err(_)`: otherwise
    pub async fn get_rating<'a>(
        params: &RatingGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Rating>> {
        let conn: &mut PgConnection = transaction_handle;

        let query = sqlx::query_as::<_, Rating>(r#"SELECT * FROM "Rating" WHERE id = $1"#)
            .bind(params.id)
            .fetch_optional(conn)
            .await?;

        if let Some(rating) = query {
            return Ok(Some(rating));
        }

        Err(DbError::from(BusinessLogicError::new(RatingDoesNotExist)))
    }

    /// Function which retrieves all ratings of user with given id, usable within a transaction
    ///
    /// # Params
    /// - `params`: structure containing the id of the user
    /// - `transaction_handle` mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - `Ok(ratings)`: on successful connection and retrieval
    /// - `Err(_)`: otherwise
    pub async fn get_user_ratings<'a>(
        params: UserGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultMultiple<Rating> {
        let conn: &mut PgConnection = transaction_handle;

        let ratings = sqlx::query_as::<_, Rating>(r#"SELECT * FROM "Rating" WHERE user_id = $1"#)
            .bind(params.id)
            .fetch_all(conn)
            .await?;

        Ok(ratings)
    }

    /// Function which retrieves all ratings for book with given id, usable within a transaction
    ///
    /// # Params
    /// - `params`: structure containing the id of the book
    /// - `transaction_handle` mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - `Ok(ratings)`: on successful connection and retrieval
    /// - `Err(_)`: otherwise
    pub async fn get_book_ratings<'a>(
        params: AudiobookGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultMultiple<Rating> {
        let conn: &mut PgConnection = transaction_handle;

        let ratings = sqlx::query_as::<_, Rating>(r#"SELECT * FROM "Rating" WHERE audiobook_id = $1"#)
            .bind(params.id)
            .fetch_all(conn)
            .await?;

        Ok(ratings)
    }

    pub async fn delete_rating<'a>(
        params: &RatingGetById,
        transaction_handle: &mut Transaction<'a, Postgres>
    ) -> DbResultSingle<Rating> {
        let conn: &mut PgConnection = transaction_handle;

        let rating = sqlx::query_as::<_, Rating>(
            r#"UPDATE "Rating" SET
            deleted_at = current_timestamp,
            edited_at = current_timestamp
            WHERE id = $1
            RETURNING *"#)
            .bind(params.id)
            .fetch_one(conn)
            .await?;

        Ok(rating)
    }

    pub async fn update<'a>(
        params: &RatingUpdate,
        transaction_handle: &mut Transaction<'a, Postgres>
    ) -> DbResultSingle<Rating> {
        let conn: &mut PgConnection = transaction_handle;

        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("UPDATE \"Rating\" SET\n");


        RatingRepository::add_to_update_query("rating=", &mut query_builder, params.rating.as_ref());
        RatingRepository::add_to_update_query("review=", &mut query_builder, params.review.as_ref());

        query_builder.push(
            "edited_at = current_timestamp\n\
         WHERE id =",
        );
        query_builder.push_bind(params.id);
        query_builder.push("\n");
        query_builder.push("RETURNING *");

        let rating = query_builder.build_query_as().fetch_one(conn).await?;
        Ok(rating)
    }

    fn add_to_update_query<'a>(
        column_name: &str,
        query_builder: &mut QueryBuilder<'a, Postgres>,
        value: Option<&'a String>,
    ) {
        if let Some(value) = value {
            query_builder.push(column_name);
            query_builder.push_bind(value);
            query_builder.push(",\n");
        }
    }

    /// Function which checks if the rating is correct (existing and not deleted)
    ///
    /// # Params
    /// - `user`: optional rating retrieved from the database
    ///
    /// # Returns
    /// - `Ok(user)`: when the rating exists and is not deleted
    /// - `Err(DbError)`: with appropriate error description otherwise
    pub fn rating_is_correct(rating: Option<Rating>) -> DbResultSingle<Rating> {
        if let Some(rating) = rating {
            if rating.deleted_at.is_none() {
                return Ok(rating);
            }
            return Err(DbError::from(BusinessLogicError::new(RatingDeleted)));
        }

        Err(DbError::from(BusinessLogicError::new(RatingDoesNotExist)))
    }
}

#[async_trait]
impl DbRepository for RatingRepository {
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
impl DbCreate<RatingCreate, Rating> for RatingRepository {

    async fn create(&mut self, data: &RatingCreate) -> DbResultSingle<Rating> {
        let rating = sqlx::query_as::<_, Rating>(
            r#"INSERT INTO \"Rating\" (user_id, audiobook_id, rating, review, created_at, edited_at, deleted_at) \
            VALUES ($1, $2, $3, $4, current_timestamp, current_timestamp, null)
            RETURNING *"#)
            .bind(&data.user_id)
            .bind(&data.audiobook_id)
            .bind(&data.rating)
            .bind(&data.review)
            .fetch_one(&*self.pool_handler.pool)
            .await?;

        Ok(rating)
    }
}

#[async_trait]
impl DbReadOne<RatingGetById, Rating> for RatingRepository {

    async fn read_one(&mut self, params: &RatingGetById) -> DbResultSingle<Rating> {
        let mut transaction = &self.pool_handler.pool.begin().await?;
        let rating = RatingRepository::get_rating(params, &mut transaction).await?;
        let rating = RatingRepository::rating_is_correct(rating);
        transaction.commit().await?;
        rating
    }
}

#[async_trait]
impl DbDelete<RatingGetById, Rating> for RatingRepository {

    async fn delete(&mut self, params: &RatingGetById) -> DbResultMultiple<Rating> {
        let mut transaction = &self.pool_handler.pool.begin().await?;
        let rating = RatingRepository::get_rating(params, &mut transaction).await?;
        if let Ok(correct_rating) = RatingRepository::rating_is_correct(rating) {
            let rating = RatingRepository::delete_rating(params, &mut transaction).await?;
            transaction.commit().await?;
            Ok(vec![rating])
        } else {
            Err(DbError::from(BusinessLogicError::new(RatingDeleted)))
        }

    }
}

#[async_trait]
impl DbUpdate<RatingUpdate, Rating> for RatingRepository {
    async fn update(&mut self, params: &RatingUpdate) -> DbResultMultiple<Rating> {
        if params.review.is_none() && params.rating.is_none() {
            return Err(DbError::from(BusinessLogicError::new(
                RatingUpdateEmpty
            )));
        }

        let mut transcation = self.pool_handler.pool.begin().await?;

        let rating = RatingRepository::get_rating(&RatingGetById{id:params.id}, &mut transcation).await?;
        RatingRepository::rating_is_correct(rating)?;

        let rating = RatingRepository::update(params, &mut transcation).await?;

        transcation.commit().await?;
        Ok(vec![rating])

    }
}



