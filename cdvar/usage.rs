use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct BoundedQueue<T> {
    inner: Mutex<VecDeque<T>>,
    not_empty: Condvar,
    not_full: Condvar,
    capacity: usize,
}

impl<T> BoundedQueue<T> {
    fn new(capacity: usize) -> Self {
        Self {
            inner: Mutex::new(VecDeque::new()),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
            capacity,
        }
    }

    fn push(&self, item: T) {
        let mut queue = self.inner.lock().unwrap();
        while queue.len() == self.capacity {
            queue = self.not_full.wait(queue).unwrap();
        }
        queue.push_back(item);
        self.not_empty.notify_one();
    }

    fn pop(&self) -> T {
        let mut queue = self.inner.lock().unwrap();
        while queue.is_empty() {
            queue = self.not_empty.wait(queue).unwrap();
        }
        let item = queue.pop_front().unwrap();
        self.not_full.notify_one();
        item
    }
}

fn main() {
    let queue = Arc::new(BoundedQueue::new(2)); // tiny capacity to force blocking

    let producer_q = Arc::clone(&queue);
    let producer = thread::spawn(move || {
        for i in 0..5 {
            producer_q.push(i);
            println!("[producer] pushed {i}");
        }
    });

    let consumer_q = Arc::clone(&queue);
    let consumer = thread::spawn(move || {
        for _ in 0..5 {
            thread::sleep(Duration::from_millis(150)); // slower consumer
            let item = consumer_q.pop();
            println!("[consumer] popped {item}");
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
