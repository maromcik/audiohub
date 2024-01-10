#[cfg(test)]
pub mod genre_repo_tests {
    use std::sync::Arc;

    use sqlx::PgPool;

    use crate::database::common::{DbCreate, DbPoolHandler, DbRepository, DbUpdate, PoolHandler};
    use crate::database::models::genre::{GenreCreate, GenreUpdate};
    use crate::database::repositories::genre::repository::GenreRepository;

    #[sqlx::test(fixtures("genres"))]
    async fn create_genre(pool: PgPool) {
        let genre_repository = GenreRepository::new(PoolHandler::new(pool));
        let u = genre_repository
            .create(&GenreCreate::new("mexicky rap"))
            .await
            .unwrap();
        assert_eq!(u.name, "mexicky rap");
        genre_repository.disconnect().await;
    }

    #[sqlx::test(fixtures("genres"))]
    async fn update_genre(pool: PgPool) {
        let genre_repository = GenreRepository::new(PoolHandler::new(pool));
        let genres = genre_repository
            .update(&GenreUpdate::new(&10, Some("audio")))
            .await
            .unwrap();
        let u = &genres[0];
        assert_eq!(u.name, "audio");
        genre_repository.disconnect().await;
    }
}
