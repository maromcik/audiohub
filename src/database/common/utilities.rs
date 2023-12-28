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
