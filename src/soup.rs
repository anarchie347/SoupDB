use std::hash::{Hash, Hasher};

use crate::Pile;

struct Ingredient {
    data: String,
}
impl Ingredient {
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl PartialEq for Ingredient {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl Eq for Ingredient {}
impl Hash for Ingredient {
    fn hash<H : Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

struct Soup {
    depths: Vec<Pile<Ingredient>>,
}

impl Soup {
    pub fn count(&self) -> usize {
        self
        .depths
        .iter()
        .map(|pile| {
            pile.len()
        })
        .sum()
    }
}



