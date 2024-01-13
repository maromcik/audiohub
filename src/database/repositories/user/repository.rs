use async_trait::async_trait;
use chrono::Utc;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::Pbkdf2;
use rand_core::OsRng;

use sqlx::{Postgres, Transaction};

use crate::database::common::error::BusinessLogicErrorKind::{
    UserDeleted, UserDoesNotExist, UserPasswordDoesNotMatch, UserUpdateParametersEmpty,
};
use crate::database::common::error::{BusinessLogicError, DbError};
use crate::database::common::error::{DbResultMultiple, DbResultSingle};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler,
};
use crate::database::models::active_audiobook::ActiveAudiobook;

use crate::database::models::bookmark::Bookmark;
use crate::database::models::user::{
    AddActiveAudiobook, BookmarkOperation, RemoveActiveAudiobook, UpdateActiveAudiobook, User,
    UserCreate, UserDelete, UserGetById, UserGetByUsername, UserLogin, UserSearch, UserUpdate,
};

fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

fn hash_password(password: String, salt: &SaltString) -> Result<String, DbError> {
    let password_hash = Pbkdf2.hash_password(password.as_bytes(), salt)?.to_string();
    Ok(password_hash)
}

fn verify_password_hash(
    expected_password_hash: &str,
    password_candidate: &str,
) -> Result<bool, DbError> {
    let parsed_hash = PasswordHash::new(expected_password_hash)?;
    let bytes = password_candidate.bytes().collect::<Vec<u8>>();
    Ok(Pbkdf2.verify_password(&bytes, &parsed_hash).is_ok())
}

#[derive(Clone)]
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
        let query = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "User"
            WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        if let Some(user) = query {
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

    pub fn verify_password(user: &User, given_password: &str) -> Result<bool, DbError> {
        verify_password_hash(&user.password_hash, given_password)
    }
}

#[async_trait]
impl DbRepository for UserRepository {
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
impl DbCreate<UserCreate, User> for UserRepository {
    /// Create a new user with the specified data
    async fn create(&self, params: &UserCreate) -> DbResultSingle<User> {
        let salt = generate_salt();
        let password_hash = hash_password(params.password.clone(), &salt)?;
        let user = sqlx::query_as!(
            User,
            r#"INSERT INTO "User" (username, email, name, surname, bio, profile_picture, password_hash, password_salt)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *"#,
            params.username,
            params.email,
            params.name,
            params.surname,
            params.bio,
            params.profile_picture,
            password_hash,
            salt.to_string()
        )
            .fetch_one(&self.pool_handler.pool)
            .await?;

        Ok(user)
    }
}

#[async_trait]
impl DbReadOne<UserLogin, User> for UserRepository {
    /// Login the user with provided parameters, if the user does not exist, is deleted or the
    /// passwords don't match, return the error about combination of email/password not working
    async fn read_one(&self, params: &UserLogin) -> DbResultSingle<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "User"
            WHERE email = $1 or username = $1
            "#,
            params.email_or_username
        )
        .fetch_optional(&self.pool_handler.pool)
        .await?;

        let user = UserRepository::user_is_correct(user)?;

        match UserRepository::verify_password(&user, &params.password) {
            Ok(ret) => {
                if ret {
                    return Ok(user);
                }
                Err(DbError::from(BusinessLogicError::new(
                    UserPasswordDoesNotMatch,
                )))
            }
            Err(e) => Err(e),
        }
    }
}

#[async_trait]
impl DbReadOne<UserGetById, User> for UserRepository {
    /// Login the user with provided parameters, if the user does not exist, is deleted or the
    /// passwords don't match, return the error about combination of email/password not working
    async fn read_one(&self, params: &UserGetById) -> DbResultSingle<User> {
        let maybe_user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "User"
            WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(&self.pool_handler.pool)
        .await?;

        let user = UserRepository::user_is_correct(maybe_user)?;
        Ok(user)
    }
}

#[async_trait]
impl DbReadOne<UserGetByUsername, User> for UserRepository {
    /// Login the user with provided parameters, if the user does not exist, is deleted or the
    /// passwords don't match, return the error about combination of email/password not working
    async fn read_one(&self, params: &UserGetByUsername) -> DbResultSingle<User> {
        let maybe_user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "User"
            WHERE username = $1
            "#,
            params.username
        )
        .fetch_optional(&self.pool_handler.pool)
        .await?;

        let user = UserRepository::user_is_correct(maybe_user)?;
        Ok(user)
    }
}

#[async_trait]
impl DbReadMany<UserSearch, User> for UserRepository {
    // ALTERNATIVE
    // async fn read_many(&self, params: &UserSearch) -> DbResultMultiple<User> {
    //     let mut query: QueryBuilder<Postgres> = QueryBuilder::new(r#" SELECT * FROM "User""#);
    //     if !params.search_fields_none() {
    //         query.push(" WHERE ");
    //     }
    //
    //     let mut query_pairs: Vec<String> = Vec::new();
    //     parse_value("username", &params.username, &mut query_pairs, None);
    //     parse_value("name", &params.name, &mut query_pairs, None);
    //     parse_value("surname", &params.surname, &mut query_pairs, None);
    //     parse_value("email", &params.email, &mut query_pairs, None);
    //
    //     add_sql_to_query(&mut query, &query_pairs, Some(" AND "));
    //
    //     println!("query: {}", query.sql());
    //     let mut transaction = self.pool_handler.pool.begin().await?;
    //
    //     let users = query
    //         .build_query_as()
    //         .fetch_all(transaction.as_mut())
    //         .await?;
    //     Ok(users)
    // }
    async fn read_many(&self, params: &UserSearch) -> DbResultMultiple<User> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "User"
            WHERE
                (username = $1 OR $1 IS NULL) AND (email = $2 OR $2 IS NULL) AND (name = $3 OR $3 IS NULL) AND (surname = $4 OR $4 IS NULL)
            "#,
            params.username,
            params.email,
            params.name,
            params.surname
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(users)
    }
}

#[async_trait]
impl DbUpdate<UserUpdate, User> for UserRepository {
    /// Update user information if we know their id (we're logged in as that user)
    /// Fails if the relevant update fields are all none
    async fn update(&self, params: &UserUpdate) -> DbResultMultiple<User> {
        if params.update_fields_none() {
            return Err(DbError::from(BusinessLogicError::new(
                UserUpdateParametersEmpty,
            )));
        }
        let mut transaction = self.pool_handler.pool.begin().await?;
        let user =
            UserRepository::get_user(UserGetById { id: params.id }, &mut transaction).await?;
        let validated_user = UserRepository::user_is_correct(user)?;
        let (password, salt) = match &params.password {
            Some(p) => {
                let salt = generate_salt();
                let password_hash = hash_password(p.clone(), &salt)?;
                (Some(password_hash), Some(salt.to_string()))
            }
            None => (None, None),
        };
        let updated_users = sqlx::query_as!(
            User,
            r#"
            UPDATE "User"
            SET
                username = COALESCE($1, username),
                email = COALESCE($2, email),
                name = COALESCE($3, name),
                surname = COALESCE($4, surname),
                bio = COALESCE($5, bio),
                profile_picture = COALESCE($6, profile_picture),
                password_hash = COALESCE($7, password_hash),
                password_salt = COALESCE($8, password_salt),
                edited_at = current_timestamp
            WHERE id = $9
            RETURNING *
            "#,
            params.username,
            params.email,
            params.name,
            params.surname,
            params.bio,
            params.profile_picture,
            password,
            salt,
            validated_user.id
        )
        .fetch_all(transaction.as_mut())
        .await?;
        transaction.commit().await?;
        Ok(updated_users)
    }
}

#[async_trait]
impl DbDelete<UserDelete, User> for UserRepository {
    /// Delete the user if we know their id (we're logged in as that user)
    async fn delete(&self, params: &UserDelete) -> DbResultMultiple<User> {
        //find user
        let mut transaction = self.pool_handler.pool.begin().await?;
        let user_query =
            UserRepository::get_user(UserGetById { id: params.id }, &mut transaction).await?;

        //user does not exist
        let _ = UserRepository::user_is_correct(user_query.clone())?;

        let users = sqlx::query_as!(
            User,
            r#"
            UPDATE "User" SET
                username = $1,
                email = $1,
                deleted_at = $2,
                edited_at = $2
            WHERE id = $1
            RETURNING *
            "#,
            params.id,
            Utc::now()
        )
        .fetch_all(transaction.as_mut())
        .await?;

        transaction.commit().await?;

        Ok(users)
    }
}

impl UserRepository {
    pub async fn get_all_active_audiobooks(
        &self,
        params: &UserGetById,
    ) -> DbResultMultiple<ActiveAudiobook> {
        let active_audiobooks = sqlx::query_as!(
            ActiveAudiobook,
            r#"
            SELECT * FROM "Active_Audiobook"
            WHERE user_id = $1
            "#,
            params.id
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(active_audiobooks)
    }

    pub async fn add_active_audiobook(
        &self,
        params: &AddActiveAudiobook,
    ) -> DbResultSingle<ActiveAudiobook> {
        let active_audiobook = sqlx::query_as!(
            ActiveAudiobook,
            r#"
            INSERT INTO "Active_Audiobook" (user_id, audiobook_id, playback_chapter_id, playback_position_in_chapter)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            params.user_id,
            params.audiobook_id,
            params.playback_chapter_id,
            params.playback_position_in_chapter
        )
            .fetch_one(&self.pool_handler.pool)
            .await?;

        Ok(active_audiobook)
    }

    pub async fn remove_active_audiobook(
        &self,
        params: &RemoveActiveAudiobook,
    ) -> DbResultSingle<ActiveAudiobook> {
        let removed_active_audiobook = sqlx::query_as!(
            ActiveAudiobook,
            r#"
            DELETE FROM "Active_Audiobook"
            WHERE user_id = $1 AND audiobook_id = $2 AND playback_chapter_id = $3
            RETURNING *
            "#,
            params.user_id,
            params.audiobook_id,
            params.playback_chapter_id,
        )
        .fetch_one(&self.pool_handler.pool)
        .await?;

        Ok(removed_active_audiobook)
    }

    pub async fn update_chapter_of_active_audiobook(
        &self,
        params: &UpdateActiveAudiobook,
    ) -> DbResultSingle<ActiveAudiobook> {
        let updated_active_audiobook = sqlx::query_as!(
            ActiveAudiobook,
            r#"
            UPDATE "Active_Audiobook"
            SET
                playback_position_in_chapter = COALESCE($1, playback_position_in_chapter)
            WHERE user_id = $2 AND audiobook_id = $3 AND playback_chapter_id = $4
            RETURNING *
            "#,
            params.playback_position_in_chapter,
            params.user_id,
            params.audiobook_id,
            params.playback_chapter_id
        )
        .fetch_one(&self.pool_handler.pool)
        .await?;

        Ok(updated_active_audiobook)
    }

    pub async fn get_all_bookmarks(&self, params: &UserGetById) -> DbResultMultiple<Bookmark> {
        let bookmarks = sqlx::query_as!(
            Bookmark,
            r#"
            SELECT * FROM "Bookmark"
            WHERE user_id = $1
            "#,
            params.id
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(bookmarks)
    }

    pub async fn bookmark(&self, params: &BookmarkOperation) -> DbResultSingle<Bookmark> {
        let bookmark = sqlx::query_as!(
            Bookmark,
            r#"
            INSERT INTO "Bookmark" (user_id, audiobook_id)
            VALUES ($1, $2)
            RETURNING *
            "#,
            params.user_id,
            params.audiobook_id
        )
        .fetch_one(&self.pool_handler.pool)
        .await?;
        Ok(bookmark)
    }

    pub async fn unbookmark(&self, params: &BookmarkOperation) -> DbResultSingle<Bookmark> {
        let bookmark = sqlx::query_as!(
            Bookmark,
            r#"
            DELETE FROM "Bookmark"
            WHERE user_id = $1 AND audiobook_id = $2
            RETURNING *
            "#,
            params.user_id,
            params.audiobook_id
        )
        .fetch_one(&self.pool_handler.pool)
        .await?;
        Ok(bookmark)
    }
}
