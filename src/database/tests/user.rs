#[cfg(test)]
pub mod user_repo_tests {
    use std::sync::Arc;

    use sqlx::PgPool;

    use crate::database::common::{DbCreate, DbPoolHandler, DbRepository, DbUpdate, PoolHandler};
    use crate::database::models::user::{UserCreate, UserUpdate};
    use crate::database::repositories::user::repository::UserRepository;

    #[sqlx::test(fixtures("users"))]
    async fn create_user(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let mut user_repository = UserRepository::new(PoolHandler::new(arc_pool));
        let u = user_repository
            .create(&UserCreate::new("pes", "p@p.com", "", "", "", "", "", ""))
            .await
            .unwrap();
        assert_eq!(u.username, "pes");
        user_repository.disconnect().await;
    }

    #[sqlx::test(fixtures("users"))]
    async fn update_user(pool: PgPool) {
        let arc_pool = Arc::new(pool);
        let mut user_repository = UserRepository::new(PoolHandler::new(arc_pool));
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
                None,
            ))
            .await
            .unwrap();
        let u = &users[0];
        assert_eq!(u.username, "doggo");
        assert_eq!(u.email, "d@d.com");
        user_repository.disconnect().await;
    }
}
