use serde::{Deserialize, Serialize};
use crate::sql::parser::ast::{Consts,Expression};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Boolean,
    Integer,
    Float,
    String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Null,
}

impl Value {
    pub fn from_expression(expr:Expression) -> Self {
        match expr{
            Expression::Consts(Consts::Boolean(b)) => Self::Boolean(b),
            Expression::Consts(Consts::Integer(i)) => Self::Integer(i),
            Expression::Consts(Consts::Float(f)) => Self::Float(f),
            Expression::Consts(Consts::String(s)) => Self::String(s),
            Expression::Consts(Consts::Null) => Self::Null,
        }
    }

    pub fn datatype(&self) -> Option<DataType> {
        match self{
            Self::Boolean(_) => Some(DataType::Boolean),
            Self::Integer(_) => Some(DataType::Integer),
            Self::Float(_) => Some(DataType::Float),
            Self::String(_) => Some(DataType::String),
            _ => None,
        }
    }
}

pub type Row = Vec<Value>;