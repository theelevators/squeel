use crate::{Statement, dml::{Command, Query}};


pub type Columns<'a> = Vec<&'a str>;

pub enum Object {
    Table(String),
    Join(String)
}


pub trait Entity: serde::de::DeserializeOwned
{

    fn as_join(cols:Columns )->Object{
        let (_, table) = Self::table_column();
        todo!("implement join")
        
    }

    fn table()->Object{
        let (_, table) = Self::table_column();
        return Object::Table(table);
    }
    fn table_column() -> (String, String);
    fn query<'a>() -> Query<'a, String, String, String> {
        let (cols, table) = Self::table_column();
         Query::new(Command::SELECT, cols, table)
    }
    fn find<'a>() -> Statement<'a, String> {
        Statement::new(Self::query().build())
    }
}
