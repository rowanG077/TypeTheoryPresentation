use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // Note that we now create a reference counter to our mutex.
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..20 {
        // We create a new Rc which points to the same data, as
        // well as increment the reference count.
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            // When the MutexGuard goes out of scope it unlocks
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
