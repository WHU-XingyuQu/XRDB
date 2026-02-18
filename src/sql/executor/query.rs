

pub struct Scan{
    table_name: String,
}

impl Scan{
    pub fn new(table_name: String) -> Box<Self>{
        Box::new(Self{table_name})
    }
}