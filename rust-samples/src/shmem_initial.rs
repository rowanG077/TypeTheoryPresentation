use std::sync::Mutex;
use std::thread;

fn main() {
    // Define a mutex to guard our data.
    //let counter = Mutex::new(0);
    let mut num = 0;

    // Define a vector to keep track of our threads.
    let mut handles = vec![];

    // Spawn 20 new threads to increment our counter.
    for _ in 0..20 {
        let handle = thread::spawn(move || {
            // Since counter : Mutex<i32> we *must* lock the mutex
            // to get the value of type i32.
            //let mut num = counter.lock().unwrap();

            for _ in 0..1000000 {
                //*num += 1;
                num += 1;
            }
        });

        // Keep track of our threads.
        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    //println!("Final counter value: {}", *counter.lock().unwrap());
    println!("Final counter value: {}", num);
}
