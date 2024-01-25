use crate::database::common::{DbPoolHandler, DbReadMany, DbRepository, PoolHandler};
use crate::database::models::genre::GenreSearch;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::genre::repository::GenreRepository;
use crate::recommender::recommandation_system::init_recommandation_system;
use sqlx::PgPool;

pub async fn init_recommender(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let audiobook_repository = AudiobookRepository::new(PoolHandler::new(pool.clone()));
    let genre_repository = GenreRepository::new(PoolHandler::new(pool.clone()));

    let books = audiobook_repository.get_all_books().await?;

    let genre_ids: Vec<i64> = books.iter().map(|book| book.genre_id).collect();
    let descriptions: Vec<&str> = books.iter().map(|book| book.description.as_str()).collect();
    let ids: Vec<i64> = books.iter().map(|book| book.id).collect();

    let get_all_genre_query = GenreSearch::new(None);
    let genres = genre_repository.read_many(&get_all_genre_query).await?;

    let genre_names: Vec<&str> = genre_ids
        .iter()
        .filter_map(|&genre_id| {
            genres
                .iter()
                .find(|&genre| genre.id == genre_id)
                .map(|genre| genre.name.as_str())
        })
        .collect();

    init_recommandation_system(descriptions, ids, genre_names).await?;
    Ok(())
}
