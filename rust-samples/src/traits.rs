fn main() {

// 3-element vector
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// Tuple
pub struct Tuple2 {
    pub first: i32,
    pub second: i32,
}


// Want a function that gives a string representation of the types.

// Would like a polymorphic function a -> string, but we can't do this because
// now nothing about the type a.


// Showable trait
// Represents: if a type implements this then we can get a string representation
pub trait Showable {
    fn show(&self) -> String;
}

// Define implementations of functions specified by the trait
impl Showable for Vector3 {
    fn show(&self) -> String {
        format!("[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl Showable for Tuple2 {
    fn show(&self) -> String {
        format!("({}, {})", self.first, self.second)
    }
}

// A function with adhoc polymorphism that works on any type as long as it has
// an implementation of the Showable trait
pub fn represent(item: &impl Showable) {
    println!("Representation: {}", item.show());
}
}
