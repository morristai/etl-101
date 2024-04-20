use std::collections::{HashMap, LinkedList};

pub struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, i32>,
    order: LinkedList<i32>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            order: LinkedList::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.cache.get(&key) {
            self.order.remove(key as usize);
            self.order.push_front(key);
            value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            self.order.remove(key as usize);
        } else if self.cache.len() >= self.capacity {
            if let Some(oldest_key) = self.order.pop_back() {
                self.cache.remove(&oldest_key);
            }
        }
        self.cache.insert(key, value);
        self.order.push_front(key);
    }
}
