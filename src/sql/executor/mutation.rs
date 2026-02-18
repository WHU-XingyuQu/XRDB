use crate::sql::types::Row;

pub struct Insert{
    table_name: String,
    columns: Vec<String>,
    rows: Vec<Row>,
}

impl Insert{
    pub fn new(table_name: String, columns: Vec<String>, rows: Vec<Row>) -> Box<Self>{
        Box::new(Self{table_name, columns, rows})
    }
}