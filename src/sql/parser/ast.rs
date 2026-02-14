use crate::sql::types::DataType;

// Abstract Syntax Tree (AST) Definition

#[derive(Debug, PartialEq)]
pub enum Statement{
    CreateTable{
        name: String,
        columns: Vec<Column>,
    },
    Insert {
        table_name: String,
        columns: Option<Vec<String>>,
        values: Vec<Vec<Expression>>,
    },
    Select{
        table_name: String,
    }
}

// Definition of Column
#[derive(Debug, PartialEq)]
pub struct Column{
    pub name: String,
    pub data_type: DataType,
    pub nullable: Option<bool>,
    pub default: Option<Expression>
}

#[derive(Debug, PartialEq)]
pub enum Expression{
    Consts(Consts),
}

impl From<Consts> for Expression {
    fn from(c: Consts) -> Self {
        Self::Consts(c)
    }
}

#[derive(Debug, PartialEq)]
pub enum Consts {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

