
use crate::objects::Columns;

use super::Query;

#[derive(Clone)]
pub struct Join<
    'a,
    C: AsRef<str> + Clone,
    T: AsRef<str> + Clone,
    J: AsRef<str> + Clone,
    E: AsRef<str>,
> {
    pub query: Query<'a, C, T, J>,
    pub to_table: E,
}

impl<
        C: AsRef<str> + Clone,
        T: AsRef<str> + Clone + std::convert::From<std::string::String>,
        E: AsRef<str> + Clone,
        J: AsRef<str> + Clone,
    > Join<'_, C, E, T, J>
{
    pub fn on(&mut self, cols: Columns) -> Query<'_, C, E, T> {
        let left_table = self.query.from.as_ref();
        let right_table = self.to_table.as_ref();

        let on = cols
            .iter()
            .map(|f| format!(" {}.{} = {}.{}", left_table, f, right_table, f))
            .collect::<Vec<String>>()
            .join(" AND ");

        let join = format!("{} ON {}", right_table, on);

        self.query.join = Some(join.into());

        return self.query.clone();
    }
}
