use ::Key;
use super::innerptr::{InnerPtr, IntoPtr};
use super::traits::JpmNode;
use ::rudymap::results::InsertResult;
use super::branch_uncompressed::BranchUncompressed;

struct Subexpanse<K: Key, V> {
    pub bitmap: u8,
    // boxes contain 16 words
    pub ptr: Option<Box<[InnerPtr<K, V>; 8]>>
}

impl<K: Key, V> Default for Subexpanse<K, V> {
    fn default() -> Subexpanse<K, V> {
        Subexpanse {
            bitmap: 0,
            ptr: None
        }
    }
}

pub struct BranchBitmap<K: Key, V> {
    subexpanses: [Subexpanse<K, V>; 8]
}

impl<K: Key, V> BranchBitmap<K, V> {
    pub fn new() -> BranchBitmap<K, V> {
        BranchBitmap {
            subexpanses: Default::default()
        }
    }
}

impl<K: Key, V> JpmNode<K, V> for BranchBitmap<K, V> {
    fn get(&self, key: &[u8]) -> Option<&V> {
        unimplemented!()
    }
    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        unimplemented!()
    }
    fn insert(&mut self, key: &[u8], value: V) -> InsertResult<V> {
        unimplemented!()
    }
    fn expand(self: Box<Self>, pop: usize, key: &[u8], value: V) -> InnerPtr<K, V> {
        unimplemented!()
    }
}
