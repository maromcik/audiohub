use std::fmt::{Debug, Display, Formatter};


#[derive(Debug, Clone)]
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
            order: Some(DbOrderColumn::default()),
            limit: Some(limit),
            offset: Some(offset)
        }
    }

    pub fn order(order: DbOrderColumn) -> Self {
        Self {
            order: Some(order),
            limit: None,
            offset: None,
        }
    }
}

impl Default for DbQueryParams {
    fn default() -> Self {
        Self {
            order: Some(DbOrderColumn::default()),
            limit: None,
            offset: None
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
            order: DbOrder::Desc
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