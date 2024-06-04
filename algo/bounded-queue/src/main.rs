mod bounded_queue;

use std::collections::HashMap;
use bounded_queue::BoundedQueue;

fn main() {
    // let queue = BoundedQueue::new(2);
    //
    // queue.push(1);
    // queue.push(2);
    // println!("{:?}", queue.pop()); // Should print Some(1)
    // queue.push(3);
    // println!("{:?}", queue.pop()); // Should print Some(2)
    // println!("{:?}", queue.pop()); // Should print Some(3)
    // println!("{:?}", queue.pop()); // Should print None

    let numbers = vec![1, 2, 3, 4, 5];

    let squared_numbers: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squared numbers: {:?}", squared_numbers);
}