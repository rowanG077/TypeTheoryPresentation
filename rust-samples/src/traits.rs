fn main() {

// Type to keep track of document information
pub struct Document {
    pub title: String,
    pub author: String,
    pub content: String,
}

// Type to keep track of a reply
pub struct Reply {
    pub username: String,
    pub content: String,
    pub to: String,
}

// Trait that allows for a summary representation
pub trait Summary {
    fn summarize(&self) -> String;
}

// Implementation of the trait for Document
impl Summary for Document {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.title, self.author, self.content)
    }
}

// Implementation of the trait for Reply
impl Summary for Reply {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Function that works on any type that implements the trait
pub fn notify(item: &impl Summary) {
    println!("Summary: {}", item.summarize());
}
}
