use crate::{Statement, dml::{Command, Query}};


pub type Columns<'a> = Vec<&'a str>;



pub trait Entity: serde::de::DeserializeOwned
{
    fn as_table() -> (String, String);
    fn to_query() -> String {
        let (cols, table) = Self::as_table();
         Query::new(Command::SELECT, cols.as_str(), table.as_str()).build()
    }
    fn find<'a>() -> Statement<'a, String> {
        Statement::new(Self::to_query())
    }
}


// pub struct IncompleteJoin<T: Entity>{
//     from_table: T,
//     from_columns: String

// }

// pub struct CompleteJoin<'a, F:Entity, T:Entity>{
//     from_table: F,
//     to_table:T,
//     on: Columns<'a>
// }

// pub struct On<'a> {
//     left_columns: Columns<'a>,
//     right_columns: Columns<'a>
// }