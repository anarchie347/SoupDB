use core::fmt;
use std::{collections::{hash_map::Keys, HashMap, HashSet}, fmt::{Debug, Error, Formatter}, hash::Hash, os::windows::thread, result};
use rand::{distributions::{self, WeightedError}, prelude::*};

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

    pub fn distinct(&self) -> Keys<'_, T, usize> {
        self.hash_table.keys()
    }

    pub fn len(&self) -> usize {
        self.hash_table.values().sum()
    }

    pub fn get_random(&self, amount: usize) -> Result<Vec<&T>, Error> {
        let kvps = self.hash_table.iter().collect::<Vec<(&T, &usize)>>();

        if kvps.len() < amount {
            return  Err(fmt::Error::());
        }

        let weights = self.hash_table.iter().map(|(k,v)| *v).collect::<Vec<usize>>();

        let mut gen = rand::thread_rng();
        let distribution = match rand::distributions::WeightedIndex::new(weights) {
            Ok(res) => res,
            Err(error) => return 
        }

        let mut indexes = HashSet::<usize>::new(); //used to ensure different items are chosen
        while indexes.len() < amount {
            indexes.insert(distribuion.sample(&mut gen));
        }

        Ok(indexes.iter().map(|i| kvps[*i].0).collect())
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