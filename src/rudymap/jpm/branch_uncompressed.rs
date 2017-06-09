use std::mem;
use std::ptr;

use super::innerptr::InnerPtr;
use super::traits::JpmNode;
use ::Key;
use ::rudymap::results::InsertResult;

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
    type OverflowNode = BranchUncompressed<K, V>;
    fn get(&self, key: &[u8]) -> Option<&V> {
        unimplemented!();
    }

    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        unimplemented!()
    }

    fn insert(&mut self, key: &[u8], value: V)
              -> InsertResult<V> {
        unimplemented!();
    }

    fn expand(self, key: &[u8], value: V) -> Box<BranchUncompressed<K, V>> {
        unimplemented!();
    }
}
