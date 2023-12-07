use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Acquire, Postgres, QueryBuilder, Transaction};

use crate::database::common::error::{DbResultMultiple, DbResultSingle};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadOne, DbRepository, DbUpdate, PoolHandler,
};
use crate::database::common::error::BusinessLogicErrorKind::{
    UserDeleted, UserDoesNotExist, UserPasswordDoesNotMatch, UserUpdateParametersEmpty,
};
use crate::database::common::error::{BusinessLogicError, DbError};
use crate::database::models::user::{User, UserLogin, UserCreate, UserDelete, UserGetById, UserUpdate};

pub struct UserRepository {
    pool_handler: PoolHandler,
}

impl UserRepository {
    /// Function which retrieves a user by their id, usable within a transaction
    ///
    /// # Params
    /// - `params`: structure containing the id of the user
    /// - `transaction_handle` mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - `Ok(user)`: on successful connection and retrieval
    /// - `Err(_)`: otherwise
    pub async fn get_user<'a>(
        params: UserGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<User>> {
        let mut tx = transaction_handle.begin().await?;

        let query = sqlx::query_as::<_, User>(r#"SELECT * FROM "User" WHERE id = $1"#)
            .bind(params.id)
            .fetch_optional(&mut *tx)
            .await?;

        if let Some(user) = query {
            tx.commit().await?;
            return Ok(Option::from(user));
        }

        Err(DbError::from(BusinessLogicError::new(UserDoesNotExist)))
    }

    /// Function which checks if the user is correct (existing and not deleted)
    ///
    /// # Params
    /// - `user`: optional user retrieved from the database
    ///
    /// # Returns
    /// - `Ok(user)`: when the user exists and is not deleted
    /// - `Err(DbError)`: with appropriate error description otherwise
    pub fn user_is_correct(user: Option<User>) -> DbResultSingle<User> {
        if let Some(user) = user {
            if user.deleted_at.is_none() {
                return Ok(user);
            }
            return Err(DbError::from(BusinessLogicError::new(UserDeleted)));
        }

        Err(DbError::from(BusinessLogicError::new(UserDoesNotExist)))
    }

    pub fn verify_password(given_password: &str, user_password: &str) -> bool {
        given_password == user_password
    }

    pub fn build_query(update_info: &UserUpdate) -> QueryBuilder<Postgres> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(r#"UPDATE "User" SET "#);

        UserRepository::add_value_to_query(&mut query_builder, &update_info.name, "name");
        UserRepository::add_value_to_query(&mut query_builder, &update_info.surname, "surname");
        UserRepository::add_value_to_query(&mut query_builder, &update_info.username, "username");
        UserRepository::add_value_to_query(&mut query_builder, &update_info.email, "email");
        UserRepository::add_value_to_query(
            &mut query_builder,
            &update_info.password_salt,
            "password_salt",
        );
        UserRepository::add_value_to_query(
            &mut query_builder,
            &update_info.password_hash,
            "password_hash",
        );
        UserRepository::add_value_to_query(&mut query_builder, &update_info.bio, "bio");
        UserRepository::add_value_to_query(
            &mut query_builder,
            &update_info.profile_picture,
            "profile_picture",
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

    pub fn add_value_to_query(
        query_builder: &mut QueryBuilder<Postgres>,
        user_value: &Option<String>,
        name: &str,
    ) {
        if let Some(val) = &user_value {
            query_builder.push(format!("{} = '{}', ", name, val));
        }
    }
}

#[async_trait]
impl DbRepository for UserRepository {
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
impl DbCreate<UserCreate, User> for UserRepository {
    /// Create a new user with the specified data
    async fn create(&mut self, data: &UserCreate) -> DbResultSingle<User> {
        let time_of_creation = Utc::now();
        let user = sqlx::query_as::<_, User>(
            r#"INSERT INTO
            "User" (username, email, name, surname, bio, profile_picture,
            password_hash, password_salt, created_at, edited_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *"#,
        )
            .bind(&data.username)
            .bind(&data.email)
            .bind(&data.name)
            .bind(&data.surname)
            .bind(&data.bio)
            .bind(&data.profile_picture)
            .bind(&data.password_hash)
            .bind(&data.password_salt)
            .bind(time_of_creation)
            .bind(time_of_creation)
            .fetch_one(&*self.pool_handler.pool)
            .await?;

        Ok(user)
    }
}

#[async_trait]
impl DbReadOne<UserLogin, User> for UserRepository {
    /// Login the user with provided parameters, if the user does not exist, is deleted or the
    /// passwords don't match, return the error about combination of email/password not working
    async fn read_one(&mut self, params: &UserLogin) -> DbResultSingle<User> {
        let user = sqlx::query_as::<_, User>(r#"SELECT * FROM "User" WHERE email = $1"#)
            .bind(&params.email)
            .fetch_optional(&*self.pool_handler.pool)
            .await?;

        let user_result = UserRepository::user_is_correct(user);
        if let Ok(user) = user_result {
            if UserRepository::verify_password(&params.password_hash, &user.password_hash) {
                return Ok(user);
            }
        }

        Err(DbError::from(BusinessLogicError::new(
            UserPasswordDoesNotMatch,
        )))
    }
}

#[async_trait]
impl DbUpdate<UserUpdate, User> for UserRepository {
    /// Update user information if we know their id (we're logged in as that user)
    /// Fails if the relevant update fields are all none
    async fn update(&mut self, params: &UserUpdate) -> DbResultMultiple<User> {
        if params.update_fields_none() {
            return Err(DbError::from(BusinessLogicError::new(
                UserUpdateParametersEmpty,
            )));
        }
        let mut transaction = self.pool_handler.pool.begin().await?;

        //finding the user
        let user_query =
            UserRepository::get_user(UserGetById { id: params.id }, &mut transaction).await?;
        let _ = UserRepository::user_is_correct(user_query.clone())?;

        //updating existing user
        let mut query_builder = UserRepository::build_query(params);
        let users = query_builder
            .build_query_as()
            .fetch_all(&mut *transaction)
            .await?;

        transaction.commit().await?;
        Ok(users)
    }
}

#[async_trait]
impl DbDelete<UserDelete, User> for UserRepository {
    /// Delete the user if we know their id (we're logged in as that user)
    async fn delete(&mut self, params: &UserDelete) -> DbResultMultiple<User> {
        //find user
        let mut transaction = self.pool_handler.pool.begin().await?;
        let user_query =
            UserRepository::get_user(UserGetById { id: params.id }, &mut transaction).await?;

        //user does not exist
        let _ = UserRepository::user_is_correct(user_query.clone())?;

        let id = params.id;
        let time_of_delete = Utc::now();

        let users = sqlx::query_as::<_, User>(
            r#"
            UPDATE "User" SET
                username = $1,
                email = $1,
                deleted_at = $2,
                edited_at = $2
            WHERE id = $1
            RETURNING *
            "#,
        )
            .bind(id)
            .bind(time_of_delete)
            .fetch_all(transaction.as_mut())
            .await?;

        let _ = sqlx::query(
            r#"
            UPDATE "Comment" SET
                deleted_at = $1,
                edited_at = $1
            WHERE commenter_id = $2
            "#,
        )
            .bind(time_of_delete)
            .bind(id)
            .execute(transaction.as_mut())
            .await?;

        transaction.commit().await?;

        Ok(users)
    }
}
