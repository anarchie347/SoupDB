use std::{collections::HashMap, hash::Hash};

///An implementaition of a multiset
pub struct Pile<T>
{
    hash_table: HashMap<T, usize>
}

impl<T> Pile<T>
where
    T : Eq + Hash
{
    pub fn new() -> Self {
        return Self {
            hash_table: HashMap::new()
        };
    }

    pub fn add(&mut self, value: T) {
        match self.hash_table.get_mut(&value) {
            None => {self.hash_table.insert(value, 1);}
            Some(i) => {*i += 1}
        }
    }

    pub fn remove(&mut self, value: T) {
        if let Some(i) = self.hash_table.get_mut(&value) {
            *i += 1;
        }
    }

}