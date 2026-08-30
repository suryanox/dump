use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let ready = Arc::new(AtomicBool::new(false));
    let ready_clone = Arc::clone(&ready);

    let producer = thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        println!("[producer] data is ready!");
        ready_clone.store(true, Ordering::SeqCst);
    });

    let consumer = thread::spawn(move || {
        while !ready.load(Ordering::SeqCst) {
           
        }
        println!("[consumer] saw it, moving on");
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
