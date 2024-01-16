use sqlx::{Postgres, QueryBuilder};
use std::fmt::Display;
use crate::database::common::query_parameters::{DbOrder, DbQueryParams};

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


pub fn generate_query_param_string(params: &Option<DbQueryParams>) -> String {
    match &params {
        None => { String::default() }
        Some(qp) => {
            let mut qp_string = String::new();
            if let Some(order) = &qp.order {
                qp_string.push_str("ORDER BY ");
                qp_string.push_str(&order.column);
                match order.order {
                    DbOrder::Asc => qp_string.push_str(" ASC"),
                    DbOrder::Desc => qp_string.push_str(" ASC")
                }
            }
            qp_string.push('\n');
            if let Some(l) = qp.limit {
                qp_string.push_str("LIMIT ");
                qp_string.push_str(l.to_string().as_str());
            }
            qp_string.push('\n');
            if let Some(o) = qp.offset {
                qp_string.push_str("OFFSET ");
                qp_string.push_str(o.to_string().as_str());
            }
            qp_string
        }
    }
}