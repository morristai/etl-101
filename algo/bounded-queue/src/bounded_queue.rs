use std::sync::{Arc, Mutex};

fn example(capacity: i32) -> Arc<BoundedQueue<i32>> {
    Arc::new(BoundedQueue::new(capacity as usize))
}

pub(crate) struct BoundedQueue<T> {
    items: Mutex<Vec<T>>,
    capacity: usize,
}

impl<T> BoundedQueue<T> {
    pub(crate) fn new(capacity: usize) -> Self {
        BoundedQueue {
            items: Mutex::new(Vec::with_capacity(capacity)),
            capacity,
        }
    }

    pub(crate) fn push(&self, item: T) -> Result<(), String> {
        let mut items = self.items.lock().unwrap();
        if items.len() == self.capacity {
            return Err("Queue is full".to_string());
        }
        items.push(item);
        Ok(())
    }

    pub(crate) fn pop(&self) -> Option<T> {
        let mut items = self.items.lock().unwrap();
        if items.is_empty() {
            None
        } else {
            Some(items.remove(0))
        }
    }
}