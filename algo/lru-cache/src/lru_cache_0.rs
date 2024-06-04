use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;

pub(crate) struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, (V, usize)>,
    counter: usize,
}

impl<K, V> LRUCache<K, V>
    where
        K: Eq + Hash + Clone,
{
    pub(crate) fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::with_capacity(capacity),
            counter: 0,
        }
    }

    pub(crate) fn get(&mut self, key: &K) -> Option<&V> {
        if let Some((value, count)) = self.map.get_mut(key) {
            self.counter += 1;
            *count = self.counter;
            Some(value)
        } else {
            None
        }
    }

    pub(crate) fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.remove(&key);
        } else if self.map.len() == self.capacity {
            let min_key = self
                .map
                .iter()
                .min_by_key(|(_, (_, count))| count)
                .map(|(k, _)| k.clone())
                .unwrap();
            self.map.remove(&min_key);
        }
        self.counter += 1;
        self.map.insert(key, (value, self.counter));
    }
}