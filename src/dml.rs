

pub enum Command {
    SELECT,
    INSERT,
    DELETE,
    UPDATE
}


pub struct Query<'a> {
    cmd: Command,
    cols: &'a str,
    limit: Option<i64>,
    from: &'a str,
    conditon: Option<&'a str>,
    join: Option<&'a str>,
}

impl Query<'_>  {
    pub fn new<'a>(cmd:Command, cols:&'a str, from_table:&'a str)->Query<'a>{
        Query { cmd, cols, limit:None,from: from_table, conditon: None, join: None }
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
                                        format!("SELECT TOP {} {} FROM {} JOIN {} WHERE {} FOR JSON PATH;",limit,self.cols, self.from, join,condition)
                                    }
                                    None =>{
                                        format!("SELECT TOP {} {} FROM {} WHERE {} FOR JSON PATH;",limit,self.cols, self.from,condition)

                                    }
                                }
                            }
                            None => {
                                match self.join {
                                    Some(join) => {
                                        format!("SELECT TOP {} {} FROM {} JOIN {} FOR JSON PATH;",limit,self.cols, self.from, join)
                                    }
                                    None =>{
                                        format!("SELECT TOP {} {} FROM {}  FOR JSON PATH;",limit,self.cols, self.from)

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
                                        format!("SELECT {} FROM {} JOIN {} WHERE {} FOR JSON PATH;",self.cols, self.from, join,condition)
                                    }
                                    None =>{
                                        format!("SELECT {} FROM {} WHERE {} FOR JSON PATH;",self.cols, self.from,condition)

                                    }
                                }
                            }
                            None => {
                                match self.join {
                                    Some(join) => {
                                        format!("SELECT {} FROM {} JOIN {} FOR JSON PATH;",self.cols, self.from, join)
                                    }
                                    None =>{
                                        format!("SELECT {} FROM {}  FOR JSON PATH;",self.cols, self.from)

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

