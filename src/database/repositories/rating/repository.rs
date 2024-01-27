use crate::database::common::error::BackendErrorKind::{
    RatingDeleted, RatingDoesNotExist, RatingUpdateParametersEmpty,
};
use crate::database::common::error::{BackendError, DbError, DbResultMultiple, DbResultSingle, EntityError};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler,
};
use crate::database::models::audiobook::AudiobookGetById;
use crate::database::models::rating::{Rating, RatingCreate, RatingGetById, RatingSearch, RatingUpdate, RatingsGetByBookId, UserRatingDisplay, DISPLAYED_RATINGS_COUNT, RatingSummaryDisplay};
use crate::database::models::user::UserGetById;

use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use crate::database::common::utilities::entity_is_correct;
use crate::database::models::Id;


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
            WHERE id = $1 AND deleted_at IS NULL
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
    #[allow(dead_code)]
    pub async fn get_user_ratings<'a>(
        params: UserGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultMultiple<Rating> {
        let ratings = sqlx::query_as!(
            Rating,
            r#"
            SELECT * FROM "Rating"
            WHERE user_id = $1 AND deleted_at IS NULL
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
    #[allow(dead_code)]
    pub async fn get_book_ratings<'a>(
        params: AudiobookGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultMultiple<Rating> {
        let ratings = sqlx::query_as!(
            Rating,
            r#"
                SELECT * FROM "Rating"
                WHERE audiobook_id = $1 AND deleted_at IS NULL
                "#,
            params.id
        )
        .fetch_all(transaction_handle.as_mut())
        .await?;

        Ok(ratings)
    }

    /// recalculate books overall rating, usable in transaction
    pub async fn update_overall_book_rating<'a>(book_id: &Id, transaction_handle : &mut Transaction<'a, Postgres>) -> DbResultSingle<()>{
        let _ = sqlx::query!(
            r#"
            UPDATE "Audiobook"
            SET overall_rating = COALESCE((
                SELECT round(AVG(R.Rating), 2)
                FROM "Rating" R
                WHERE R.audiobook_id = $1 AND R.deleted_at IS NULL
            ), 0)
            WHERE id = $1

            "#,
            book_id
        ).execute(transaction_handle.as_mut()).await?;
        Ok(())
    }

    /// Function which retrieves rating for book with given id for given user, usable within a transaction
    ///
    /// # Params
    /// - `params`: structure containing the id of the book
    /// - `transaction_handle` mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - `Ok(ratings)`: on successful connection and retrieval
    /// - `Err(_)`: otherwise
    pub async fn get_user_book_rating<'a>(&self,
        audiobook_id: &Id,
        user_id: &Id,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Rating>> {
        let rating = sqlx::query_as!(
            Rating,
            r#"
                SELECT * FROM "Rating"
                WHERE audiobook_id = $1 AND user_id = $2 AND deleted_at IS NULL
                "#,
            audiobook_id,
            user_id,
        )
            .fetch_optional(transaction_handle.as_mut())
            .await?;

        Ok(rating)
    }

    pub async fn get_rating_count(&self, book_id: &Id) -> DbResultSingle<i64>{
        let record = sqlx::query!(
            r#"
            SELECT COUNT(*) as count FROM "Rating"
            WHERE audiobook_id = $1 AND deleted_at IS NULL
            "#,
            book_id,
        ).fetch_one(&self.pool_handler.pool).await?;

        Ok(record.count.unwrap_or(0))
    }

    pub async fn delete_rating<'a>(
        params: &RatingGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Rating> {
        let rating = sqlx::query_as!(
            Rating,
            r#"
            DELETE FROM "Rating"
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

    async fn create_transactional<'a>(params: &RatingCreate, transaction_handle: &mut Transaction<'a, Postgres>,) -> DbResultSingle<Rating> {
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
            .fetch_one(transaction_handle.as_mut())
            .await?;

        Ok(rating)
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
        entity_is_correct(rating, EntityError::new(RatingDeleted, RatingDoesNotExist), false)
    }

    pub async fn create_or_update_displayed_rating(&self, params: &RatingCreate) -> DbResultSingle<UserRatingDisplay> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let existing_rating = self.get_user_book_rating(&params.audiobook_id, &params.user_id, &mut transaction).await?;
        let rating_id : Id;
        if let Some(review) = existing_rating {
            let rating_params = RatingUpdate {
                id: review.id,
                rating: params.rating,
                review: params.review.to_owned(),
            };
            rating_id = RatingRepository::update(&rating_params, &mut transaction).await?[0].id;

        } else {
            rating_id = RatingRepository::create_transactional(params, &mut transaction).await?.id;
        }

        RatingRepository::update_overall_book_rating(&params.audiobook_id, &mut transaction).await?;

        let displayed_rating = sqlx::query_as!(
            UserRatingDisplay,
            r#"
            SELECT R.audiobook_id AS book_id, U.name AS user_name, U.surname AS user_surname, R.rating AS rating,
                COALESCE(R.review, '') AS review, R.created_at AS created_at, U.profile_picture AS user_thumbnail,
                U.id AS user_id
            FROM "Rating" R LEFT JOIN "User" U ON R.user_id = U.id
            WHERE R.id = $1 AND R.deleted_at IS NULL
            "#,
            rating_id
        ).fetch_one(transaction.as_mut())
            .await?;


        transaction.commit().await?;

        Ok(displayed_rating)
    }

    pub async fn get_ratings_display(&self, params: &RatingSearch) -> DbResultMultiple<UserRatingDisplay> {
        let ratings = sqlx::query_as!(
            UserRatingDisplay,
            r#"
            SELECT R.audiobook_id AS book_id, U.name AS user_name, U.surname AS user_surname, R.rating AS rating,
                R.review AS review, R.created_at AS created_at, U.profile_picture AS user_thumbnail, U.id AS user_id
            FROM "User" U JOIN "Rating" R ON R.user_id = U.id
            WHERE
                (R.audiobook_id = $1 OR $1 IS NULL)
                AND (R.user_id = $2 OR $2 IS NULL)
                AND (R.rating >= $3 OR $3 IS NULL)
                AND (R.rating <= $4 OR $4 IS NULL)
                AND (R.review = $5 OR $5 IS NULL)
                AND R.deleted_at IS NULL
            ORDER BY R.created_at DESC
            LIMIT $6
            OFFSET COALESCE($7, 0)
            "#,
            params.audiobook_id,
            params.user_id,
            params.min_rating,
            params.max_rating,
            params.review,
            DISPLAYED_RATINGS_COUNT as i64,
            params.offset
        ).fetch_all(&self.pool_handler.pool).await?;

        Ok(ratings)
    }

    /// Returns data for displaying overall ratings of given book. Star count vector contains number of
    /// ratings with that many stars for given index -> star_count[0] = number of ratings with 0 stars ...
    pub async fn get_rating_summary(&self, book_id: &Id) -> DbResultSingle<RatingSummaryDisplay>{
        let summary = sqlx::query!(
            r#"
            SELECT rating AS stars, COUNT(*) AS star_count
            FROM "Rating"
            WHERE audiobook_id = $1 AND deleted_at IS NULL
            GROUP BY rating
            ORDER BY rating
            "#,
            book_id
        ).fetch_all(&self.pool_handler.pool).await?;

        let count_row = sqlx::query!(
            r#"
            SELECT overall_rating
            FROM "Audiobook"
            WHERE id=$1 AND deleted_at IS NULL
            "#,
            book_id
        ).fetch_one(&self.pool_handler.pool).await?;


        let mut star_count : Vec<i64> = vec![];
        let mut all_ratings_count = 0;
        for i in 0..6 {
            let count = summary.iter().find(|rec| rec.stars == i);
            match count {
                Some(count) => {
                    let count = count.star_count.unwrap_or(0);
                    all_ratings_count += count;
                    star_count.push(count);
                },
                None => star_count.push(0)
            }
        }

        let rating_summary_display = RatingSummaryDisplay{
            all_ratings_count,
            star_count,
            overall_rating: count_row.overall_rating,
        };

        Ok(rating_summary_display)
    }

    pub async fn delete_rating_for_book(&self,
        book_id: &Id,
        user_id: &Id,
    ) -> DbResultSingle<Rating> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let rating = sqlx::query_as!(
            Rating,
            r#"
            UPDATE "Rating"
            SET
                deleted_at = current_timestamp,
                edited_at = current_timestamp
            WHERE audiobook_id = $1 AND user_id = $2
            RETURNING *
            "#,
            book_id,
            user_id
        )
            .fetch_one(transaction.as_mut())
            .await?;

        RatingRepository::update_overall_book_rating(book_id, &mut transaction).await?;
        transaction.commit().await?;
        Ok(rating)
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
