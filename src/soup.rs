struct Data {
    value: String,
}

struct Ingredient {
    data: Data,
}

struct Soup {
    depths: pile::Pile<Ingredient>,
}

impl Soup {
    fn count(&self) -> usize {
        self
        .depths
        .iter()
        .map(|ingredient| {

        })
    }
}

impl Ingredient {
    fn size(&self) -> usize {
        self.data.value.len()
    }
}
