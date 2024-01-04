use crate::database::models::Id;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Bookmark {
    pub user_id: Id,
    pub audiobook_id: Id,
}
