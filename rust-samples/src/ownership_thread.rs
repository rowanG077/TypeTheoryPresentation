use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // The same as ownership_borrow.rs except that the 'move' keyword is given.
    let handle = thread::spawn(move || {
        // The lifetime of the original thread is no longer an issue since this
        // closure is the sole owner.
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
