use super::object::IndexObject;

use std::collections::{btree_map, BTreeMap};

#[derive(Debug)]
pub struct Indexes {
    store: BTreeMap<u64, IndexObject>,
}

impl Indexes {
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, index: IndexObject) {
        self.store.insert(index.0, index);
    }

    pub fn get(&self, offset: u64) -> Option<IndexObject> {
        self.store.get(&offset).cloned()
    }

    pub fn remove(&mut self, offset: u64) {
        self.store.remove(&offset);
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    pub fn iter(&self) -> btree_map::Iter<u64, IndexObject> {
        self.store.iter()
    }
}
