use crate::database::models::utilities::get_default_profile_picture;
use crate::database::models::Id;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Id,
    // --------------
    pub username: String,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub bio: String,
    pub profile_picture: Option<String>,
    pub password_hash: String,
    pub password_salt: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

pub struct UserDisplay {
    pub id: Id,
    // --------------
    pub username: String,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub bio: String,
    pub profile_picture: String,
    pub password_hash: String,
    pub password_salt: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
}

impl From<User> for UserDisplay {
    fn from(value: User) -> Self {
        Self {
            profile_picture: get_default_profile_picture(&value.profile_picture),
            id: value.id,
            username: value.username,
            email: value.email,
            name: value.name,
            surname: value.surname,
            bio: value.bio,
            password_hash: value.password_hash,
            password_salt: value.password_salt,
            created_at: value.created_at,
            edited_at: value.edited_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub bio: String,
    pub profile_picture: Option<String>,
    pub password: String,
}

impl UserCreate {
    #[must_use]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        username: &str,
        email: &str,
        name: &str,
        surname: &str,
        password: &str,
        bio: &str,
        profile_picture: Option<&str>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            username: username.to_owned(),
            email: email.to_owned(),
            name: name.to_owned(),
            surname: surname.to_owned(),
            password: password.to_owned(),
            bio: bio.to_owned(),
            profile_picture: profile_picture.and_then(change_to_owned),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UserSearch {
    pub username: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
}

impl UserSearch {
    #[must_use]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        username: Option<&str>,
        email: Option<&str>,
        name: Option<&str>,
        surname: Option<&str>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            username: username.and_then(change_to_owned),
            email: email.and_then(change_to_owned),
            name: name.and_then(change_to_owned),
            surname: surname.and_then(change_to_owned),
        }
    }
    #[must_use]
    pub const fn search_fields_none(&self) -> bool {
        self.username.is_none()
            && self.email.is_none()
            && self.name.is_none()
            && self.surname.is_none()
    }
}

/// Structure passed to the repository when trying to update a user
#[derive(Debug, Clone)]
pub struct UserUpdate {
    pub id: Id,
    pub username: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
    pub password: Option<String>,
}

impl UserUpdate {
    #[must_use]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &Id,
        username: Option<&str>,
        email: Option<&str>,
        name: Option<&str>,
        surname: Option<&str>,
        bio: Option<&str>,
        profile_picture: Option<&str>,
        password_hash: Option<&str>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: *id,
            username: username.and_then(change_to_owned),
            email: email.and_then(change_to_owned),
            name: name.and_then(change_to_owned),
            surname: surname.and_then(change_to_owned),
            bio: bio.and_then(change_to_owned),
            profile_picture: profile_picture.and_then(change_to_owned),
            password: password_hash.and_then(change_to_owned),
        }
    }

    #[must_use]
    pub const fn update_fields_none(&self) -> bool {
        self.username.is_none()
            && self.email.is_none()
            && self.name.is_none()
            && self.surname.is_none()
            && self.bio.is_none()
            && self.profile_picture.is_none()
            && self.password.is_none()
    }
}

/// Structure passed to the repository when trying to delete a user
#[derive(Debug, Clone)]
pub struct UserDelete {
    pub id: Id,
}

impl UserDelete {
    #[must_use]
    #[inline]
    pub const fn new(id: &Id) -> Self {
        Self { id: *id }
    }
}

/// Structure passed to the repository when trying to log in (read one == login)
#[derive(Debug, Clone, Deserialize)]
pub struct UserLogin {
    pub email_or_username: String,
    pub password: String,
}

impl UserLogin {
    #[must_use]
    #[inline]
    pub fn new(email: &str, password_hash: &str) -> Self {
        Self {
            email_or_username: email.to_owned(),
            password: password_hash.to_owned(),
        }
    }
}

/// Structure passed to the repository when trying to find a user (generic function) for
/// transactions which check whether the specified user exists
#[derive(Debug, Clone)]
pub struct UserGetById {
    pub id: Id,
}

#[derive(Debug, Clone)]
pub struct UserGetByUsername {
    pub username: String,
}

impl UserGetByUsername {
    #[must_use]
    #[inline]
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_owned(),
        }
    }
}

impl UserGetById {
    #[must_use]
    #[inline]
    pub const fn new(id: &Id) -> Self {
        Self { id: *id }
    }
}

#[derive(Debug, Clone)]
pub struct UserUpdatePassword {
    pub id: Id,
    pub old_password: String,
    pub new_password: String,
}

impl UserUpdatePassword {
    pub fn new(id: &Id, old_password: &str, new_password: &str) -> Self {
        Self {
            id: *id,
            old_password: old_password.to_owned(),
            new_password: new_password.to_owned(),
        }
    }
}
