extern crate odbc;
extern crate r2d2;

use anyhow::Error;
use odbc::odbc_safe::AutocommitOn;
use odbc::ResultSetState::{Data, NoData};
use odbc::{Connection, Statement};
use r2d2::{Pool, PooledConnection};
use r2d2_odbc::ODBCConnectionManager;

pub type SQLPool = Pool<ODBCConnectionManager>;
pub type SQLPooledConnection = PooledConnection<ODBCConnectionManager>;
pub type SQLConnection<'a> = Connection<'a, AutocommitOn>;


pub enum DatabaseResponse {
    Results(String),
    Message(String),
}

pub fn establish_connection(odbc_conn: &str) -> Result<SQLPool, Error> {
    return Ok(init_pool(odbc_conn)?);
}

pub fn init_pool(odbc_conn: &str) -> Result<SQLPool, r2d2::Error> {
    return Pool::builder().build(ODBCConnectionManager::new(odbc_conn));
}
pub fn sql_pool_handler(pool: &SQLPool) -> Result<SQLPooledConnection, Error> {
    return Ok(pool.get()?)
}
pub struct Query<'a> {
   pub cmd: &'a str,
   pub params: Option<Vec<&'a str>>,
}

impl Query<'_> {
    pub fn new(cmd: &str)->Query{
        Query{cmd, params:None}
    }

    pub fn new_parametized<'a>(cmd:&'a str, params:Vec<&'a str>)->Query<'a>{
        Query{cmd, params:Some(params)}
    }

    pub fn execute<'env>(&mut self, conn: &SQLConnection<'env>) -> Result<DatabaseResponse, Error> {
        let mut stmt = Statement::with_parent(conn)?;
        let mut results: Vec<_> = vec![];
        match self.params.take() {
            Some(params) => {
                for param in &params {
                    let idx = params.iter().position(|e| e == param).unwrap();
                    let tmp = stmt.bind_parameter::<&str>((idx + 1) as u16, &param);
                    stmt = tmp?;
                }

                match stmt.exec_direct(self.cmd) {
                    Ok(Data(mut stmt)) => {
                        let cols = stmt.num_result_cols()?;
                        while let Some(mut cursor) = stmt.fetch()? {
                            let mut row_data: Vec<_> = vec![];
                            for i in 1..cols + 1 {
                                match cursor.get_data::<&str>(i as u16)? {
                                    Some(val) => row_data.push(val.to_string()),
                                    None => row_data.push("NULL".to_string()),
                                }
                            }
                            results.push(row_data.join("\t"));
                        }
                        return Ok(DatabaseResponse::Results(results.join("\n")));
                    }
                    Ok(NoData(_)) => {
                        return Ok(DatabaseResponse::Message(
                            "Query complete.".to_string(),
                        ))
                    }
                    Err(err) => {
                        return Err(Error::msg(String::from_utf8(
                            err.get_raw_message().to_vec(),
                        )?))
                    }
                }
            }

            None => match stmt.exec_direct(self.cmd) {
                Ok(Data(mut stmt)) => {
                    let cols = stmt.num_result_cols()?;
                    while let Some(mut cursor) = stmt.fetch()? {
                        let mut row_data: Vec<_> = vec![];
                        for i in 1..cols + 1 {
                            match cursor.get_data::<&str>(i as u16)? {
                                Some(val) => row_data.push(val.to_string()),
                                None => row_data.push("NULL".to_string()),
                            }
                        }
                        results.push(row_data.join("\t"));
                    }
                    return Ok(DatabaseResponse::Results(results.join("\n")));
                }
                Ok(NoData(_)) => {
                    return Ok(DatabaseResponse::Message(
                        "Query complete.".to_string(),
                    ))
                }
                Err(err) => {
                    return Err(Error::msg(String::from_utf8(
                        err.get_raw_message().to_vec(),
                    )?))
                }
            },
        }
    }
}
