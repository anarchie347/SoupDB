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

impl Soup {
    fn count(&self) -> u32 {
        self.count()
    }
}
