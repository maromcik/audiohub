use crate::database::models::Id;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct AudiobookAuthor {
    pub author_id: Id,
    pub audiobook_id: Id,
}
