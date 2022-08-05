use std::{thread, time};

struct Item {
    name: String,
    description: String,
    rating: i8,
    category: Categories,
    price: f32
}

enum Categories {
    Toy,
    Office,
    Gaming,
    Clothing,
    Vehicle,
    Gadget,
    Other
}

fn sleep(s: u64) {
    thread::sleep(time::Duration::from_secs(s));
}

fn main() {
    
}
