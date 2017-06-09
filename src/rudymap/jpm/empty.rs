use super::traits::JpmNode;
use super::innerptr::InnerPtr;
use ::Key;
use super::leaf_linear::LeafLinear;
use ::rudymap::results::InsertResult;

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
    type OverflowNode = LeafLinear<K, V>;
    fn get(&self, key: &[u8]) -> Option<&V> {
        None
    }

    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        unimplemented!()
    }

    fn insert(&mut self, key: &[u8], value: V) -> InsertResult<V> {
        InsertResult::Resize(value)
    }

    fn expand(self, key: &[u8], value: V) -> Box<LeafLinear<K, V>> {
        let mut leaf_linear = Box::new(LeafLinear::new());
        leaf_linear.insert(key, value).success();
        leaf_linear
    }
}
