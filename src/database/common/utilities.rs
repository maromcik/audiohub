use crate::database::common::query_parameters::{DbOrder, DbQueryParams};
use sqlx::{Postgres, QueryBuilder};
use std::fmt::Display;

pub fn add_sql_to_query(
    query_builder: &mut QueryBuilder<Postgres>,
    query_pairs: &Vec<String>,
    delimiter: Option<&str>,
) {
    let delim = delimiter.unwrap_or("");
    let sql = query_pairs.join(delim);
    query_builder.push(sql);
}

pub fn parse_value<T>(
    name: &str,
    val: &Option<T>,
    query_pairs: &mut Vec<String>,
    operator: Option<&str>,
) where
    T: Display,
{
    let Some(value) = &val else {
        return;
    };
    let op = operator.unwrap_or("=");
    query_pairs.push(format!(r#"{name} {op} '{value}'"#));
}

pub fn generate_query_param_string(params: &DbQueryParams) -> String {
    let mut qp_string = String::new();
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
