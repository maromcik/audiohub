use crate::database::models::Id;
use crate::error::AppError;
use actix_identity::Identity;

pub fn parse_user_id(identity: Identity) -> Result<Id, AppError> {
    Ok(identity.id()?.parse::<i64>()?)
}
