//! A Hashtable

use datastructures::linkedlist::*;

struct HashTable<T> {
    count: usize,
    table: Box<LinkedList<T>>,
}

impl<T> HashTable<T> {
    pub fn size(&self) -> usize {
        self.count
    }
}
