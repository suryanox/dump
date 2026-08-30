use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    let producer = thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        thread::sleep(Duration::from_secs(2));

        let mut ready = lock.lock().unwrap();
        *ready = true;
        println!("[producer] data is ready, ringing the doorbell");
        cvar.notify_one();
    });

    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*pair;
        let mut ready = lock.lock().unwrap();

        while !*ready {
            ready = cvar.wait(ready).unwrap();
        }
        println!("[consumer] woke up because the flag is true");
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
