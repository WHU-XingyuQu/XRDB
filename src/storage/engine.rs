use crate::error::Result;

use std::ops::{Bound, RangeBounds};
use crate::sql::types::Row;

// abstract storage interface, for memory and disk
pub trait Engine {
    type EngineIterator<'a>: EngineIterator
    where
       Self: 'a;

    // 设置 key/value
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<()>;

    // 获取 key 对应的数据
    fn get(&mut self, key: Vec<u8>) -> Result<Option<Vec<u8>>>;

    // 删除 key 对应的数据，如果 key 不存在的话则忽略
    fn delete(&mut self, key: Vec<u8>) -> Result<()>;

    fn scan(&mut self, range: impl RangeBounds<Vec<u8>>) -> Self::EngineIterator<'_>;

    fn scan_prefix(&mut self, prefix: Vec<u8>) -> Self::EngineIterator<'_>{
        // For example: To find all strings with prefix "aaa", use range [aaa, aab)
        let start = Bound::Included(prefix.clone());
        let mut bound_prefix=prefix.clone();
        if let Some(last) = bound_prefix.iter_mut().last() {
            *last+=1;
        }
        let end = Bound::Excluded(bound_prefix);

        self.scan((start,end))
    }

}

pub trait EngineIterator:DoubleEndedIterator<Item=Result<(Vec<u8>, Vec<u8>)>> {}

