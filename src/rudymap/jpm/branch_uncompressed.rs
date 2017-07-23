use std::mem;
use std::ptr;

use super::innerptr::InnerPtr;
use super::traits::JpmNode;
use ::Key;
use ::rudymap::results::{InsertResult, RemoveResult};

pub struct BranchUncompressed<K: Key, V> {
    array: [InnerPtr<K, V>; 256]
}

impl<K: Key, V> BranchUncompressed<K, V> {
    pub fn new() -> BranchUncompressed<K, V> {
        unsafe {
            let mut branch = BranchUncompressed {
                array: { mem::uninitialized() }
            };
            for position in branch.array.iter_mut() {
                ptr::write(position, InnerPtr::empty());
            }
            branch
        }
    }
}

impl<K: Key, V> JpmNode<K, V> for BranchUncompressed<K, V> {
    fn get(&self, key: &[u8]) -> Option<&V> {
        let (&byte, subkey) = key.split_first().unwrap();
        self.array[byte as usize].get(subkey)
    }

    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        let (&byte, subkey) = key.split_first().unwrap();
        self.array[byte as usize].get_mut(subkey)
    }

    fn insert(&mut self, key: &[u8], value: V) -> InsertResult<V> {
        let (&byte, subkey) = key.split_first().unwrap();
        let evicted = self.array[byte as usize].insert(subkey, value);
        InsertResult::Success(evicted)
    }

    fn expand(self, population: usize, key: &[u8], value: V) -> InnerPtr<K, V> {
        unreachable!()
    }

    fn remove(&mut self, key: &[u8]) -> RemoveResult<V> {
        let (&byte, subkey) = key.split_first().unwrap();
        let evicted = self.array[byte as usize].remove(subkey);
        RemoveResult::Success(evicted)
    }

    fn shrink_remove(self, pop: usize, key: &[u8]) -> (InnerPtr<K, V>, V) {
        unreachable!()
    }

    fn memory_usage(&self) -> usize {
        let mut bytes = mem::size_of::<Self>();
        for jpm in self.array.iter() {
            bytes += jpm.target_memory_usage();
        }
        bytes
    }
}
