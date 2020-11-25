use std::thread;
use std::time::Duration;

fn main() {
    // Create a new thread.
    let handle = thread::spawn(|| {
        // Print a string nine times.
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Print four more strings.
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for the thread to finish.
    handle.join().unwrap();
}
