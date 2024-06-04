// For linked list
#![feature(linked_list_remove)]

mod lru_cache_1;
// mod lru_cache_0;

use lru_cache_1::LRUCache;

fn main() {
    let mut cache = LRUCache::new(2);

    cache.put(1, 1);
    cache.put(2, 2);
    println!("{:?}", cache.get(1)); // Should print Some(1)
    cache.put(3, 3);
    println!("{:?}", cache.get(2)); // Should print None
}
