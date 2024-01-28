#[cfg(test)]
pub mod user_repo_tests {

    use sqlx::PgPool;

    use crate::database::common::{
        DbCreate, DbPoolHandler, DbReadMany, DbRepository, DbUpdate, PoolHandler,
    };
    use crate::database::models::user::{UserCreate, UserSearch, UserUpdate};
    use crate::database::repositories::user::repository::UserRepository;

    #[sqlx::test(fixtures("users"))]
    async fn create_user(pool: PgPool) {
        let user_repository = UserRepository::new(PoolHandler::new(pool));
        let u = user_repository
            .create(&UserCreate::new(
                "cokel",
                "cok@cok.com",
                "",
                "",
                "",
                "",
                None,
            ))
            .await
            .expect("Create user should succeed");
        assert_eq!(u.username, "cokel");
        user_repository.disconnect().await;
    }

    #[sqlx::test(fixtures("users"))]
    async fn update_user(pool: PgPool) {
        let user_repository = UserRepository::new(PoolHandler::new(pool));
        let users = user_repository
            .update(&UserUpdate::new(
                &9,
                Some("doggo"),
                Some("d@d.com"),
                None,
                None,
                None,
                None,
                None,
            ))
            .await
            .expect("Update user should succeed");
        let u = &users[0];
        assert_eq!(u.username, "doggo");
        assert_eq!(u.email, "d@d.com");
        user_repository.disconnect().await;
    }

    #[sqlx::test(fixtures("users"))]
    async fn get_filtered_users(pool: PgPool) {
        let user_repository = UserRepository::new(PoolHandler::new(pool));
        let users = user_repository
            .read_many(&UserSearch::new(None, None, None, None))
            .await
            .expect("Read many users should succeed");
        assert_eq!(users.len(), 9);

        let users = user_repository
            .read_many(&UserSearch::new(Some("pes"), None, None, Some("Hafski")))
            .await
            .expect("Read many users with filters should succeed");
        assert_eq!(users.len(), 1);

        let u = &users[0];
        assert_eq!(u.username, "pes");
        assert_eq!(u.email, "pe@pe.com");
        user_repository.disconnect().await;
    }
}
