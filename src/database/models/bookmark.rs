use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct AudiobookUser {
    pub user_id: Uuid,
    pub audiobook_id: Uuid,
}
