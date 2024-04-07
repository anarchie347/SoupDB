fn main() {
    println!("Hello, world!");
}

struct Data {
    value: String,
}

struct Ingredient {
    data: Data,
    size: u32,
    depth: u32,
}

struct Soup {
    data: Vec<Ingredient>,
}
