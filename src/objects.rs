use crate::{
    dml::{complex::Join, Command, Query},
    Statement,
};

pub type Cols = Vec<&'static str>;
pub type Tbl = &'static str;

#[derive(Debug,Clone)]
pub struct Table {
    pub name: Tbl,
    pub cols: Cols,
}
#[derive(Debug,Clone)]
pub struct Columns {
    pub cols: Cols,
}

pub trait Entity: serde::de::DeserializeOwned {
    fn as_join(cols: Cols) -> Join {
        let (columns, table) = Self::table_column();
        return Join::from_right(
            Table {
                name: &table,
                cols: columns,
            },
            cols,
        );
    }
    fn table() -> Table {
        let (cols, table) = Self::table_column();
        return Table { name: &table, cols };
    }
    fn table_column() -> (Vec<&'static str>, &'static str);
    fn query() -> Query {
        let (cols, table) = Self::table_column();
        let table = Table {
            name: &table,
            cols: cols.clone(),
        };
        return Query::new(Command::SELECT, cols, table);
    }
    fn find<'a>() -> Statement<'a, String> {
        Statement::new(Self::query().build())
    }
}
