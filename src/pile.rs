use core::fmt;
use std::{collections::HashMap, fmt::{Debug, Error, Formatter}, hash::Hash};

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

    pub fn remove(&mut self, value: &T) {
        if let Some(i) = self.hash_table.get_mut(value) {
            match i {
                0 => {self.hash_table.remove(value);}
                _ => {*i -= 1}
            }
        }
    }

}

impl<T> Debug for Pile<T>
where
    T : Eq + Hash + fmt::Debug
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "Pile {{")?;
        for key in self.hash_table.keys() {
            for _ in 0..self.hash_table[key] {
                write!(formatter, "{:?}, ", key)?;
            }
        }
        write!(formatter, "}} ")
    }
}