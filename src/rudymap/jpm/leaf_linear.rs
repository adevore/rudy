use super::traits::JpmNode;
use super::innerptr::InnerPtr;
use ::rudymap::results::{InsertResult, RemoveResult};
use ::Key;
use super::leaf_bitmap::LeafBitmap;
use std::marker::PhantomData;

pub struct LeafLinear<K: Key, V> {
    /*
    keys: [u8; <K as Key>::SIZE * 4],
    values: [V; 4],
    */
    pd: PhantomData<(K, V)>
}

impl<K: Key, V> LeafLinear<K, V> {
    pub fn new() -> LeafLinear<K, V> {
        unimplemented!();
        /*
        LeafLinear {
            keys: [0; K::SIZE * 4],
            values:
        }
        */
    }
}

impl<K: Key, V> JpmNode<K, V> for LeafLinear<K, V> {
    fn get(&self, key: &[u8]) -> Option<&V> {
        unimplemented!();
    }

    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        unimplemented!()
    }

    fn insert(&mut self, key: &[u8], value: V) -> InsertResult<V> {
        unimplemented!();
    }

    fn expand(self, pop: usize, key: &[u8], value: V) -> InnerPtr<K, V> {
        unimplemented!();
    }

    fn remove(&mut self, key: &[u8]) -> RemoveResult<V> {
        unimplemented!();
    }

    fn shrink_remove(self, pop: usize, key: &[u8]) -> (InnerPtr<K, V>, V) {
        unreachable!()
    }
}
