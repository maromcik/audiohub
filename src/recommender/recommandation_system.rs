use crate::recommender_grpc_api::model_ai_client::ModelAiClient;
use crate::recommender_grpc_api::{
    AddBookRequest, BooksCollection, DeleteBookRequest, SimilarRequest,
};

const URL: &str = "audiohub-ai-svc:50051";

pub async fn init_recommendation_system(
    book_bios: Vec<&str>,
    book_ids: Vec<i64>,
    book_genres: Vec<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ModelAiClient::connect(URL).await?;

    let request = tonic::Request::new(BooksCollection {
        bios: book_bios.iter().map(|x| x.to_string()).collect(),
        ids: book_ids,
        genres: book_genres.iter().map(|x| x.to_string()).collect(),
    });

    client.init(request).await?;
    Ok(())
}

pub async fn add_book_to_recommendation_system(
    book_bio: &str,
    book_id: i64,
    book_genre: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ModelAiClient::connect(URL).await?;

    let request = tonic::Request::new(AddBookRequest {
        bio: book_bio.to_string(),
        id: book_id,
        genre: book_genre.to_string(),
    });

    client.add_book(request).await?;
    Ok(())
}

pub async fn delete_book_from_recommendation(
    book_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ModelAiClient::connect(URL).await?;

    let request = tonic::Request::new(DeleteBookRequest { id: book_id });

    client.delete_book(request).await?;
    Ok(())
}

pub async fn recommend_books(
    book_bio: &str,
    book_id: i64,
    book_genre: &str,
    count: i32,
) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let mut client = ModelAiClient::connect(URL).await?;

    let request = tonic::Request::new(SimilarRequest {
        id: book_id,
        bio: book_bio.to_string(),
        genre: book_genre.to_string(),
        count,
    });

    let response = client.recommend_books(request).await?;
    let found_ids = response.into_inner().ids;
    Ok(found_ids)
}
