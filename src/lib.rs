extern crate odbc;
extern crate r2d2;

use anyhow::Error;
use odbc::odbc_safe::AutocommitOn;
use odbc::Connection;
use odbc::ResultSetState::{Data, NoData};
use r2d2::{Pool, PooledConnection};
use r2d2_odbc::ODBCConnectionManager;
use serde::de::DeserializeOwned;

pub type SQLPool = Pool<ODBCConnectionManager>;
pub type SQLPooledConnection = PooledConnection<ODBCConnectionManager>;
pub type SQLConnection<'a> = Connection<'a, AutocommitOn>;

pub fn establish_connection(odbc_conn: &str) -> Result<SQLPool, Error> {
    return Ok(init_pool(odbc_conn)?);
}

pub fn init_pool(odbc_conn: &str) -> Result<SQLPool, r2d2::Error> {
    return Pool::builder().build(ODBCConnectionManager::new(odbc_conn));
}
pub fn sql_pool_handler(pool: &SQLPool) -> Result<SQLPooledConnection, Error> {
    return Ok(pool.get()?);
}
pub trait Entity
where
    Self: Sized + DeserializeOwned,
{
    fn as_table() -> String;
    fn find<'a>() -> Statement<'a, String> {
        Statement::new(Self::as_table())
    }
}

pub enum DatabaseResponse {
    Results(String),
    Message(String),
    Error(String),
}

#[derive(Clone)]
pub struct Statement<'a, S: AsRef<str>> {
    cmd: S,
    pub params: Option<Vec<&'a str>>,
}

impl<S: std::convert::AsRef<str>> Statement<'_, S> {
    pub fn new<'a>(cmd: S) -> Statement<'a, S> {
        Statement { cmd, params: None }
    }

    pub fn new_parametized<'a>(cmd: S, params: Vec<&'a str>) -> Statement<'a, S> {
        Statement {
            cmd,
            params: Some(params),
        }
    }
    pub fn fetch<'env>(&mut self, dbpool: &SQLPool) -> DatabaseResponse {
        let binding = sql_pool_handler(dbpool).unwrap();
        let conn: &odbc::Connection<'_, odbc::odbc_safe::AutocommitOn> = binding.raw();
        match self.execute(conn) {
            Ok(response) => response,
            Err(error) => DatabaseResponse::Error(error.to_string()),
        }
    }

    pub fn all<T: Entity>(mut self,dbpool: &SQLPool) -> Result<Vec<T>, Error> {
        match self.fetch(dbpool) {
            DatabaseResponse::Results(results) => Ok(serde_json::from_str(&results).unwrap()),
            DatabaseResponse::Message(msg) => panic!("{}", msg),
            DatabaseResponse::Error(error) => panic!("{}", error),
        }
    }

    pub fn execute<'env>(&mut self, conn: &SQLConnection<'env>) -> Result<DatabaseResponse, Error> {
        let mut stmt = odbc::Statement::with_parent(conn)?;
        let mut results: Vec<_> = vec![];
        match self.params.take() {
            Some(params) => {
                for param in &params {
                    let idx = params.iter().position(|e| e == param).unwrap();
                    let tmp = stmt.bind_parameter::<&str>((idx + 1) as u16, &param);
                    stmt = tmp?;
                }

                match stmt.exec_direct(self.cmd.as_ref()) {
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
                        return Ok(DatabaseResponse::Message("Query complete.".to_string()))
                    }
                    Err(err) => {
                        return Err(Error::msg(String::from_utf8(
                            err.get_raw_message().to_vec(),
                        )?))
                    }
                }
            }

            None => match stmt.exec_direct(self.cmd.as_ref()) {
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
                    return Ok(DatabaseResponse::Message("Query complete.".to_string()))
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

