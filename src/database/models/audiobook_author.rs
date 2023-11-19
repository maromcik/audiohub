use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct AudiobookAuthor {
    pub author_id: Uuid,
    pub audiobook_id: Uuid,
}
