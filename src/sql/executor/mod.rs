use crate::sql::types::Row;

mod mutation;
mod query;
mod schema;

// divide different realization of tasks into different files
// mutation: Insert, Update, ... , to amend data
// query: scan, ..., to read data
// schema: create table ...

//pub trait Executor<T:Transaction>{
//
//}

// the set of execution's results
#[derive(Debug)]
pub enum ResultSet{
    CreateTable{
        table_name: String,
    },
    Insert{
        count: usize, // time of execution
    },
    Scan{
        table_name: String,
        columns: Vec<String>,
        rows: Vec<Row>,
    },
}