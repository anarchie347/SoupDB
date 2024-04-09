struct Data {
    value: String,
}

struct Ingredient {
    data: Data,
    depth: usize,
}

struct Soup {
    depths: Vec<Ingredient>,
}

impl Soup {
    fn count(&self) -> usize {
        self.data.len()
    }
}

impl Ingredient {
    fn size(&self) -> usize {
        self.data.value.len()
    }
}
