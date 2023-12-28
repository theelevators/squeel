use crate::{objects::{Object, Entity}, Statement, SQLPool};

use self::complex::Join;

pub mod complex;
pub mod simple;

#[derive(Clone)]
pub enum Command {
    SELECT,
    INSERT,
    DELETE,
    UPDATE
}

#[derive(Clone)]
pub struct Query<'a,C: AsRef<str>,T: AsRef<str>, J: AsRef<str> > where C: Clone, T: Clone {
    cmd: Command,
    cols: C,
    limit: Option<i64>,
    from: T,
    conditon: Option<&'a str>,
    join: Option<J>,
}

impl<C: AsRef<str> + Clone,T: AsRef<str>+ Clone, J: AsRef<str>+ Clone>  Query<'_, C, T, J>  {
    pub fn new<'a>(cmd:Command, cols:C, from_table:T)->Query<'a, C, T,J>{
        Query { cmd, cols, limit:None,from: from_table, conditon: None, join: None }
    }

    pub fn run<E: Entity>(self,dbpool: &SQLPool)->Result<Vec<E>, anyhow::Error>{
        Statement::new(self.build()).all(dbpool)

    }
    
    pub fn join(&mut self, table:Object)->Join<'_, C, T,J,String,>{

        let to_table = match table {
            Object::Table(table) => table,
            _ => panic!("Invalid Object")
        };        
        
        let join = 

        Join{
            query:self.clone(),
            to_table
        };

        join
    }


    pub fn build(self)->String{

        match self.cmd {
            Command::SELECT => {

                match self.limit {
                    Some(limit) => {
                        match self.conditon {
                            Some(condition)=>{

                                match self.join {
                                    Some(join) => {
                                        format!("SELECT TOP {} {} FROM {} JOIN {} WHERE {} FOR JSON PATH;",limit,self.cols.as_ref(), self.from.as_ref(), join.as_ref(),condition)
                                    }
                                    None =>{
                                        format!("SELECT TOP {} {} FROM {} WHERE {} FOR JSON PATH;",limit,self.cols.as_ref(), self.from.as_ref(),condition)

                                    }
                                }
                            }
                            None => {
                                match self.join {
                                    Some(join) => {
                                        format!("SELECT TOP {} {} FROM {} JOIN {} FOR JSON PATH;",limit,self.cols.as_ref(), self.from.as_ref(), join.as_ref())
                                    }
                                    None =>{
                                        format!("SELECT TOP {} {} FROM {}  FOR JSON PATH;",limit,self.cols.as_ref(), self.from.as_ref())

                                    }
                                }

       
                            }
                        }
                        
                    }
                    None => {

                        match self.conditon {
                            Some(condition)=>{

                                match self.join {
                                    Some(join) => {
                                        format!("SELECT {} FROM {} JOIN {} WHERE {} FOR JSON PATH;",self.cols.as_ref(), self.from.as_ref(), join.as_ref(),condition)
                                    }
                                    None =>{
                                        format!("SELECT {} FROM {} WHERE {} FOR JSON PATH;",self.cols.as_ref(), self.from.as_ref(),condition)

                                    }
                                }
                            }
                            None => {
                                match self.join {
                                    Some(join) => {
                                        format!("SELECT {} FROM {} JOIN {} FOR JSON PATH;",self.cols.as_ref(), self.from.as_ref(), join.as_ref())
                                    }
                                    None =>{
                                        format!("SELECT {} FROM {}  FOR JSON PATH;",self.cols.as_ref(), self.from.as_ref())

                                    }
                                }

       
                            }
                        }
                        


                    }

                }

                
            }
            _ => {
                panic!("command not supported")
            }
        }


    }
}

