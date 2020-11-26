use std::str::FromStr;

// This function borrows an i32
fn borrow_str(borrowed_str: &String) {
    println!("This borrowed String is: {}", borrowed_str);
}

// This function takes ownership of an i32
fn ownership_str(owned_str: String) {
    println!("This owned String is: {}", owned_str);
}

fn main() {
    let val = String::from_str("hello world").unwrap();

    // Borrow the value note the & indicating a borrow
    // Can be done as many times as you want.
    borrow_str(&val);
    //borrow_str(&val);
    //borrow_str(&val);

    // Move ownership to function ownership_str
    ownership_str(val);

    // Now value has been moved by ownership_str
    // We can no longer use it.
    //borrow_str(&val);
}
