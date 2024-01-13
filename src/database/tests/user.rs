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
            .create(&UserCreate::new("cokel", "c@c.com", "", "", "", "", ""))
            .await
            .unwrap();
        assert_eq!(u.username, "cokel");
        user_repository.disconnect().await;
    }

    #[sqlx::test(fixtures("users"))]
    async fn update_user(pool: PgPool) {
        let user_repository = UserRepository::new(PoolHandler::new(pool));
        let users = user_repository
            .update(&UserUpdate::new(
                &10,
                Some("doggo"),
                Some("d@d.com"),
                None,
                None,
                None,
                None,
                None,
            ))
            .await
            .unwrap();
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
            .unwrap();
        assert_eq!(users.len(), 2);

        let users = user_repository
            .read_many(&UserSearch::new(Some("pes"), None, None, Some("Hafski")))
            .await
            .unwrap();
        assert_eq!(users.len(), 1);

        let u = &users[0];
        assert_eq!(u.username, "pes");
        assert_eq!(u.email, "p@p.com");
        user_repository.disconnect().await;
    }
}
