use std::thread;

fn main() {
    // Define a simple vector with three elements.
    let v = vec![1, 2, 3];

    // The closure captures 'v' from the environment, Rust infers that 'v'
    // should be borrowed because of the println function.
    let handle = thread::spawn(|| {
        // However, since 'v' is only borrowed the new thread can't know if
        // something would happen to 'v' in the other thread.
        println!("Here's a vector: {:?}", v);
    });

    // Wait for the thread to finish.
    handle.join().unwrap();
}
