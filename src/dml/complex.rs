use crate::objects::{Cols, Table};

#[derive(Debug, Clone)]
pub enum JoinState {
    Complete,
    Incomplete,
}

#[derive(Debug, Clone)]
pub struct Join {
    pub state: JoinState,
    pub l_tbl: Option<Table>,
    pub r_tbl: Option<Table>,
    pub l_col: Option<Cols>,
    pub r_col: Option<Cols>,
}

impl Join {
    pub fn new(l_tbl: Table, l_col: Cols, r_tbl: Table, r_col: Cols) -> Join {
        Join {
            state: JoinState::Complete,
            l_tbl: Some(l_tbl),
            r_tbl: Some(r_tbl),
            l_col: Some(l_col),
            r_col: Some(r_col),
        }
    }

    pub fn from_right(r_tbl: Table, r_col: Cols) -> Join {
        Join {
            state: JoinState::Incomplete,
            l_tbl: None,
            r_tbl: Some(r_tbl),
            l_col: None,
            r_col: Some(r_col),
        }
    }
    pub fn from_left(l_tbl: Table, l_col: Cols) -> Join {
        Join {
            state: JoinState::Incomplete,
            l_tbl: Some(l_tbl),
            r_tbl: None,
            l_col: Some(l_col),
            r_col: None,
        }
    }

    pub fn build(self) -> String {
        match self.state {
            JoinState::Complete => {
                let l_tbl = self.l_tbl.as_ref().unwrap().name;
                let r_tbl = self.r_tbl.as_ref().unwrap().name;

                let on = self
                    .l_col
                    .unwrap()
                    .iter()
                    .zip(self.r_col.unwrap())
                    .enumerate()
                    .map(|(idx, (l, r))| match idx {
                        0 => format!("ON {l_tbl}.{l} = {r_tbl}.{r}"),
                        _ => format!(" AND {l_tbl}.{l} = {r_tbl}.{r}")
                    } )
                    .collect::<Vec<_>>()
                    .join("");

                format!("JOIN {l_tbl} {on}")
            }
            JoinState::Incomplete => {
                panic!("Cannot build an incomplete join.")
            }
        }
    }
}
