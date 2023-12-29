use serde::de::DeserializeOwned;

use crate::{
    objects::{Cols,Table},
    SQLPool, Statement,
};

use self::complex::{Join, JoinState};

pub mod complex;
pub mod simple;

#[derive(Debug,Clone)]
pub enum Command {
    SELECT,
    INSERT,
    DELETE,
    UPDATE,
}

#[derive(Debug,Clone)]
pub struct Query
{
    cmd: Command,
    cols: Cols,
    limit: Option<i64>,
    from: Table,
    conditon: Option<String>,
    join: Option<Join>,
}

impl Query {
    pub fn new(cmd: Command, cols: Cols, from_table: Table) -> Query {
        Query {
            cmd,
            cols,
            limit: None,
            from: from_table,
            conditon: None,
            join: None,
        }
    }

    pub fn all<E: DeserializeOwned >(self, dbpool: &SQLPool) -> Result<Vec<E>, anyhow::Error> {

        println!("{}", self.clone().build());
        Statement::new(self.build()).all(dbpool)
    }

    pub fn limit(mut self, limit:i64)->Query{
        self.limit = Some(limit);
        self
    }
    
    pub fn join(mut self, join: Join) -> Query {
        self.join = Some(join);
        let mut join = self.join.expect("No join has been provided");
        join.state = JoinState::Complete;
        self.join = Some(join);
        self
    }



    pub fn on(mut self, cols: Cols) -> Query {
        let mut join = self.join.take().expect("No join has been provided");

        if join.l_tbl.is_none() & join.r_tbl.is_none() {
            panic!("No tables have been provided");
        }

        if  join.l_tbl.is_none() & join.r_tbl.is_some(){
            join.l_tbl = Some(self.from.clone());

        }else if join.r_tbl.is_none() & join.l_tbl.is_some(){
            join.r_tbl = Some(self.from.clone());

        }

        if  join.l_col.is_none() & join.r_col.is_some(){
            join.l_col = Some(cols);

        }else if join.r_col.is_none() & join.l_col.is_some(){
            join.r_col = Some(cols);

        }
        self.join = Some(join);
        self
    }

    pub fn build(self) -> String {
        match self.cmd {
            Command::SELECT => {
                match self.limit {
                    Some(limit) => {
                        match self.conditon {
                            Some(condition) => {
                                match self.join {
                                    Some(join) => {
                                        format!("SELECT TOP {} {} FROM {} {} WHERE {} FOR JSON PATH;",limit,self.cols.join(", "), &self.from.name, join.build(),condition)
                                    }
                                    None => {
                                        format!(
                                            "SELECT TOP {} {} FROM {} WHERE {} FOR JSON PATH;",
                                            limit,
                                            self.cols.join(", "),
                                            &self.from.name,
                                            condition
                                        )
                                    }
                                }
                            }
                            None => match self.join {
                                Some(join) => {
                                    format!(
                                        "SELECT TOP {} {} FROM {} {} FOR JSON PATH;",
                                        limit,
                                        self.cols.join(", "),
                                        &self.from.name,
                                        join.build()
                                    )
                                }
                                None => {
                                    format!(
                                        "SELECT TOP {} {} FROM {} FOR JSON PATH;",
                                        limit,
                                        self.cols.join(", "),
                                        &self.from.name
                                    )
                                }
                            },
                        }
                    }
                    None => match self.conditon {
                        Some(condition) => match self.join {
                            Some(join) => {
                                format!(
                                    "SELECT {} FROM {} JOIN {} WHERE {} FOR JSON PATH;",
                                    self.cols.join(", "),
                                    &self.from.name,
                                    join.build(),
                                    condition
                                )
                            }
                            None => {
                                format!(
                                    "SELECT {} FROM {} WHERE {} FOR JSON PATH;",
                                    self.cols.join(", "),
                                    &self.from.name,
                                    condition
                                )
                            }
                        },
                        None => match self.join {
                            Some(join) => {
                                format!(
                                    "SELECT {} FROM {} {} FOR JSON PATH;",
                                    self.cols.join(", "),
                                    &self.from.name,
                                    join.build()
                                )
                            }
                            None => {
                                format!(
                                    "SELECT {} FROM {} FOR JSON PATH;",
                                    self.cols.join(", "),
                                    &self.from.name
                                )
                            }
                        },
                    },
                }
            }
            _ => {
                panic!("command not supported")
            }
        }
    }
}
