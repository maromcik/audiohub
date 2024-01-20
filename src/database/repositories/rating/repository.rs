use crate::database::common::error::BackendErrorKind::{
    RatingDeleted, RatingDoesNotExist, RatingUpdateParametersEmpty,
};
use crate::database::common::error::{BackendError, DbError, DbResultMultiple, DbResultSingle};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler,
};
use crate::database::models::audiobook::AudiobookGetById;
use crate::database::models::rating::{Rating, RatingCreate, RatingGetById, RatingSearch, RatingUpdate, RatingsGetByBookId, UserRatingDisplay};
use crate::database::models::user::UserGetById;

use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

#[derive(Clone)]
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
        let query = sqlx::query_as!(
            Rating,
            r#"
            SELECT * FROM "Rating"
            WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        if let Some(rating) = query {
            return Ok(Some(rating));
        }

        Err(DbError::from(BackendError::new(RatingDoesNotExist)))
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
        let ratings = sqlx::query_as!(
            Rating,
            r#"
            SELECT * FROM "Rating"
            WHERE user_id = $1
            "#,
            params.id
        )
        .fetch_all(transaction_handle.as_mut())
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
        let ratings = sqlx::query_as!(
            Rating,
            r#"
                SELECT * FROM "Rating"
                WHERE audiobook_id = $1
                "#,
            params.id
        )
        .fetch_all(transaction_handle.as_mut())
        .await?;

        Ok(ratings)
    }

    pub async fn delete_rating<'a>(
        params: &RatingGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Rating> {
        let rating = sqlx::query_as!(
            Rating,
            r#"
            UPDATE "Rating"
            SET
                deleted_at = current_timestamp,
                edited_at = current_timestamp
            WHERE id = $1
            RETURNING *
            "#,
            params.id
        )
        .fetch_one(transaction_handle.as_mut())
        .await?;

        Ok(rating)
    }

    pub async fn update<'a>(
        params: &RatingUpdate,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultMultiple<Rating> {
        let ratings = sqlx::query_as!(
            Rating,
            r#"
            UPDATE "Rating"
            SET
                rating = COALESCE($1, rating),
                review = COALESCE($2, review),
                edited_at = current_timestamp
            WHERE id = $3
            RETURNING *
            "#,
            params.rating,
            params.review,
            params.id,
        )
        .fetch_all(transaction_handle.as_mut())
        .await?;

        Ok(ratings)
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
            return Err(DbError::from(BackendError::new(RatingDeleted)));
        }

        Err(DbError::from(BackendError::new(RatingDoesNotExist)))
    }

    pub async fn create_displayed_rating(&self, params: &RatingCreate) -> DbResultSingle<UserRatingDisplay> {
        let rating = self.create(params).await?;
        let displayed_rating = sqlx::query_as!(
            UserRatingDisplay,
            r#"
            SELECT R.audiobook_id AS book_id, U.name AS user_name, U.surname AS user_surname, R.rating AS rating,
                COALESCE(R.review, '') AS review, R.created_at AS created_at, U.profile_picture AS user_thumbnail
            FROM "Rating" R LEFT JOIN "User" U ON R.user_id = U.id
            WHERE R.id = $1
            "#,
            rating.id
        ).fetch_one(&self.pool_handler.pool)
            .await?;

        Ok(displayed_rating)
    }

    pub async fn get_ratings_display(&self, params: &RatingSearch) -> DbResultMultiple<UserRatingDisplay> {
        let ratings = sqlx::query_as!(
            UserRatingDisplay,
            r#"
            SELECT R.audiobook_id AS book_id, U.name AS user_name, U.surname AS user_surname, R.rating AS rating,
                R.review AS review, R.created_at AS created_at, U.profile_picture AS user_thumbnail
            FROM "User" U JOIN "Rating" R ON R.user_id = U.id
            WHERE
                (R.audiobook_id = $1 OR $1 IS NULL)
                AND (R.user_id = $2 OR $2 IS NULL)
                AND (R.rating >= $3 OR $3 IS NULL)
                AND (R.rating <= $4 OR $4 IS NULL)
                AND (R.review = $5 OR $5 IS NULL)
            ORDER BY R.created_at DESC
            "#,
            params.audiobook_id,
            params.user_id,
            params.min_rating,
            params.max_rating,
            params.review,
        ).fetch_all(&self.pool_handler.pool).await?;

        Ok(ratings)
    }
}

#[async_trait]
impl DbRepository for RatingRepository {
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
impl DbCreate<RatingCreate, Rating> for RatingRepository {
    async fn create(&self, params: &RatingCreate) -> DbResultSingle<Rating> {
        let rating = sqlx::query_as!(
            Rating,
            r#"
            INSERT INTO "Rating" (user_id, audiobook_id, rating, review)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            params.user_id,
            params.audiobook_id,
            params.rating,
            params.review
        )
        .fetch_one(&self.pool_handler.pool)
        .await?;

        Ok(rating)
    }
}

#[async_trait]
impl DbReadOne<RatingGetById, Rating> for RatingRepository {
    async fn read_one(&self, params: &RatingGetById) -> DbResultSingle<Rating> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let rating = RatingRepository::get_rating(params, &mut transaction).await?;
        let rating = RatingRepository::rating_is_correct(rating);
        transaction.commit().await?;
        rating
    }
}

#[async_trait]
impl DbReadMany<RatingSearch, Rating> for RatingRepository {
    async fn read_many(&self, params: &RatingSearch) -> DbResultMultiple<Rating> {
        let ratings = sqlx::query_as!(
            Rating,
            r#"
            SELECT * FROM "Rating"
            WHERE
                (audiobook_id = $1 OR $1 IS NULL)
                AND (user_id = $2 OR $2 IS NULL)
                AND (rating >= $3 OR $3 IS NULL)
                AND (rating <= $4 OR $4 IS NULL)
                AND (review = $5 OR $5 IS NULL)
            "#,
            params.audiobook_id,
            params.user_id,
            params.min_rating,
            params.max_rating,
            params.review,
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(ratings)
    }
}

#[async_trait]
impl DbReadMany<RatingsGetByBookId, Rating> for RatingRepository {
    async fn read_many(&self, params: &RatingsGetByBookId) -> DbResultMultiple<Rating> {
        let ratings = sqlx::query_as!(
            Rating,
            r#"
            SELECT * FROM "Rating"
            WHERE
                audiobook_id = $1
            ORDER BY created_at DESC
            "#,
            params.audiobook_id,
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(ratings)
    }
}

#[async_trait]
impl DbDelete<RatingGetById, Rating> for RatingRepository {
    async fn delete(&self, params: &RatingGetById) -> DbResultMultiple<Rating> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let rating = RatingRepository::get_rating(params, &mut transaction).await?;
        if let Ok(_) = RatingRepository::rating_is_correct(rating) {
            let rating = RatingRepository::delete_rating(params, &mut transaction).await?;
            transaction.commit().await?;
            Ok(vec![rating])
        } else {
            Err(DbError::from(BackendError::new(RatingDeleted)))
        }
    }
}

#[async_trait]
impl DbUpdate<RatingUpdate, Rating> for RatingRepository {
    async fn update(&self, params: &RatingUpdate) -> DbResultMultiple<Rating> {
        if params.review.is_none() {
            return Err(DbError::from(BackendError::new(
                RatingUpdateParametersEmpty,
            )));
        }

        let mut transcation = self.pool_handler.pool.begin().await?;

        let rating =
            RatingRepository::get_rating(&RatingGetById { id: params.id }, &mut transcation)
                .await?;
        RatingRepository::rating_is_correct(rating)?;

        let ratings = RatingRepository::update(params, &mut transcation).await?;

        transcation.commit().await?;
        Ok(ratings)
    }
}
