#![allow(unused)]

use std::collections::HashMap;
use std::hash::Hash;

pub struct MultiMap<K, V> {
    map: HashMap<K, Vec<V>>
}

impl<K, V> MultiMap<K, V> where K: Eq + Hash {
    pub fn new() -> Self {
        MultiMap { map: HashMap::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let pos = self.map.entry(key).or_insert_with(|| Vec::new());
        pos.push(value);
    }

    pub fn get(&self, key: &K) -> &[V] {
        self.map.get(key).expect("should be in MultiMap dict!")
    }

    pub fn get_values(&self) -> Vec<(&K, &[V])> {
        self.map.iter().map(|(k, v)| (k, v.as_slice())).collect()
    }
}

