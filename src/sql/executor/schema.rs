use crate::sql::schema::Table;

pub struct CreateTable{
    schema: Table,
}

impl CreateTable{
    pub fn new(schema: Table) -> Box<Self>{
        Box::new(Self{schema})
    }
}