use std::sync::{Arc, Mutex};
use std::thread;

const INCREMENTS_PER_THREAD: u32 = 1000;
const THREADS: u32 = 10;

// Here's the safe way to increment a shared counter in Rust. Of course we
// could've used a Mutex to fix the race condition in Python too. The key
// difference here is that Rust *forces* us to use this Mutex. An unsynchronized
// implementation won't even compile!
#[allow(dead_code)]
fn good_increment(counter: Arc<Mutex<u32>>) {
    *counter.lock().unwrap() += 1;
}

// If you jump through hoops, you can make Rust reproduce the race condition
// from Python. You have to take the lock, copy the counter, *give up* the lock,
// take it *again*, and finally write the incremented value.
fn bad_increment(counter: Arc<Mutex<u32>>) {
    let value = *counter.lock().unwrap();
    // The MutexGuard above isn't locally bound, so it's immediately dropped,
    // and we're free to take it again...for SCIENCE.
    *counter.lock().unwrap() = value + 1;
}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        let counter_ref = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..INCREMENTS_PER_THREAD {
                bad_increment(counter_ref.clone());
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Expected: {}", THREADS * INCREMENTS_PER_THREAD);
    println!("Actual: {}", *counter.lock().unwrap());
}
