use super::traits::JpmNode;
use super::innerptr::{InnerPtr, IntoPtr};
use ::Key;
//use super::leaf_linear::LeafLinear;
use super::leaf_bitmap::LeafBitmap;
use super::branch_uncompressed::BranchUncompressed;
use super::branch_linear::BranchLinear;
use ::rudymap::results::{InsertResult, RemoveResult};

use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct Empty<K: Key, V> {
    phantom: PhantomData<(K, V)>,
}

impl<K: Key, V> Empty<K, V> {
    pub fn new() -> Empty<K, V> {
        Empty {
            phantom: PhantomData
        }
    }
}

impl<K: Key, V> JpmNode<K, V> for Empty<K, V> {
    fn get(&self, key: &[u8]) -> Option<&V> {
        None
    }

    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        None
    }

    fn insert(&mut self, key: &[u8], value: V) -> InsertResult<V> {
        InsertResult::Resize(value)
    }

    fn expand(self, pop: usize, key: &[u8], value: V) -> InnerPtr<K, V> {
        //let mut leaf_linear = Box::new(LeafLinear::new());
        //leaf_linear.insert(key, value).success();
        //leaf_linear
        if key.len() == 1 {
            let mut leaf = Box::new(LeafBitmap::new());
            leaf.insert(key, value).success();
            IntoPtr::into_ptr(leaf, pop)
        } else {
            let mut branch = Box::new(BranchLinear::new());
            branch.insert(key, value).success();
            IntoPtr::into_ptr(branch, pop)
        }
    }

    fn remove(&mut self, key: &[u8]) -> RemoveResult<V> {
        RemoveResult::Success(None)
    }

    fn shrink_remove(self, pop: usize, key: &[u8]) -> (InnerPtr<K, V>, V) {
        unreachable!();
    }
}
