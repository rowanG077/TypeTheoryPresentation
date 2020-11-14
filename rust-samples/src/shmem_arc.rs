use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Replace Rc with Arc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..20 {
        // Replace Rc with Arc
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            for _ in 0..1000000 {
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}
