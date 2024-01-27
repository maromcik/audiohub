use crate::database::common::{DbPoolHandler, DbReadMany, DbRepository, PoolHandler};
use crate::database::models::genre::GenreSearch;
use crate::database::repositories::audiobook::repository::AudiobookRepository;
use crate::database::repositories::genre::repository::GenreRepository;
use crate::recommender::recommandation_system::{add_book_to_recommandation_system, init_recommandation_system, recommend_books};
use sqlx::PgPool;
use crate::database::models::audiobook::Audiobook;

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

pub async fn add_book_recommender(audiobook: &Audiobook, genre_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bio = audiobook.description.as_str();
    let id = audiobook.id;

    add_book_to_recommandation_system(bio, id, genre_name).await?;
    Ok(())
}
