use std::sync::mpsc;
use std::thread;

fn main() {
    // We create a channel for a thread to communicate over.
    let (tx_h, rx) = mpsc::channel();

    // Clone the transmitter we have a second one for the second thread.
    let tx_w = tx_h.clone();

    thread::spawn(move || {
        let hello = String::from("hello");

        tx_h.send(hello).unwrap();

        // Cannot use hello here because they have been moved
        println!("value of hello: {}", hello);
    });

    thread::spawn(move || {
        let world = String::from("world");

        tx_w.send(world).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
    // Or we can also manually call recv
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
}
