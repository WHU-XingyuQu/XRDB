use std::{
    collections::{BTreeMap, HashSet},
    sync::{Arc, Mutex, MutexGuard},
    u64,
};
use super::engine::Engine;

pub struct Mvcc<E: Engine> {
    engine: Arc<Mutex<E>>, // thread safe
}