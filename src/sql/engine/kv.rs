use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    sql::{
        schema::Table,
        types::{Row, Value},
    },
    storage::{self, engine::Engine as StorageEngine},
};

use super::{Engine, Transaction};

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Key{
    Table(String),
    Row(String, Value),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum KeyPrefix{
    Table,
    Row(String),
}