use crate::database::common::error::{BackendError, DbError, DbResultSingle, EntityError};
use crate::database::common::query_parameters::{BookState, DbOrder, DbQueryParams};
use crate::database::common::HasDeletedAt;
use crate::CONSIDER_AUDIOBOOK_FINISHED_PERCENTAGE;

pub fn generate_query_param_string(params: &DbQueryParams) -> String {
    let ratio = CONSIDER_AUDIOBOOK_FINISHED_PERCENTAGE / 100f64;
    let mut qp_string = String::new();
    if !params.fetch_deleted {
        qp_string.push_str("AND a.deleted_at IS NULL\n");
    }
    if let Some(state) = &params.book_state {
        match state {
            BookState::Finished(val) => {
                qp_string.push_str(
                    format!("AND ((ab.playback_position / a.length > {ratio}) = {val})\n").as_str(),
                );
            }
            BookState::Fresh(val) => {
                qp_string.push_str(format!("AND (ab.audiobook_id IS NULL = {val})\n").as_str());
            }
            BookState::Active(val) => {
                qp_string.push_str(
                    format!("AND ((ab.playback_position / a.length <= {ratio}) = {val})\n")
                        .as_str(),
                );
            }
        }
    }

    if let Some(order) = &params.order {
        qp_string.push_str("ORDER BY ");
        qp_string.push_str(&order.column);
        match order.order {
            DbOrder::Asc => qp_string.push_str(" ASC"),
            DbOrder::Desc => qp_string.push_str(" DESC"),
        }
    }
    qp_string.push('\n');
    if let Some(l) = params.limit {
        qp_string.push_str("LIMIT ");
        qp_string.push_str(l.to_string().as_str());
    }
    qp_string.push('\n');
    if let Some(o) = params.offset {
        qp_string.push_str("OFFSET ");
        qp_string.push_str(o.to_string().as_str());
    }
    qp_string
}

pub fn entity_is_correct<T: HasDeletedAt>(
    entity: Option<T>,
    error: EntityError,
    fetch_deleted: bool,
) -> DbResultSingle<T> {
    if let Some(ent) = entity {
        if fetch_deleted || !ent.is_deleted() {
            return Ok(ent);
        }
        return Err(DbError::from(BackendError::new(error.deleted)));
    }

    Err(DbError::from(BackendError::new(error.does_not_exist)))
}
