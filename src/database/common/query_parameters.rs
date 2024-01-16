use std::fmt::{Debug, Display, Formatter};


#[derive(Debug, Clone, Default)]
pub struct DbQueryParams {
    pub order: Option<DbOrderColumn>,
    pub limit: Option<i64>,
    pub offset: Option<i64>
}

impl DbQueryParams {
    pub fn new(
        order: Option<DbOrderColumn>,
        limit: Option<i64>,
        offset: Option<i64>
    ) -> Self {
        Self {
            order,
            limit,
            offset
        }
    }

    pub fn limit(limit: i64, offset: i64) -> Self {
        Self {
            order: None,
            limit: Some(limit),
            offset: Some(offset)
        }
    }
}

#[derive(Debug, Clone)]
pub struct DbOrderColumn {
    pub column: String,
    pub order: DbOrder
}

impl DbOrderColumn {
    pub fn new(column: &str, order: DbOrder) -> Self {
        Self {
            column: column.to_owned(),
            order
        }
    }
}

impl Default for DbOrderColumn {
    fn default() -> Self {
        Self {
            column: "created_at".to_string(),
            order: DbOrder::Asc
        }
    }
}

#[derive(Clone)]
pub enum DbOrder {
    Asc,
    Desc,
}

impl DbOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbOrder::Asc => write!(f, "ASC"),
            DbOrder::Desc => write!(f, "DESC"),
        }
    }
}


impl Display for DbOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for DbOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}
