use std::sync::mpsc;
use std::thread;

fn main() {
    // We create a channel for a thread to communicate over.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let hello = String::from("hello");
        let world = String::from("world");

        // Note that sending can go wrong if the reciever
        // has been dropped. We ignore this here panic if
        // this is the case
        tx.send(hello).unwrap();
        tx.send(world).unwrap();

        // Cannot use hello or world here anymore because they have been moved
        // println!("value of hello: {}, value of world: {}", hello, world);
    });

    for received in rx {
        println!("Got: {}", received);
    }
    // Or we can also manually call recv
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
}
