use super::traits::JpmNode;
use super::innerptr::InnerPtr;
use ::rudymap::results::InsertResult;
use ::Key;
use super::leaf_bitmap::LeafBitmap;

pub struct LeafLinear<K, V> {
    keys: PackedArray<K>,
    values: [V; 4]
}

struct PackedArray<T> where T: Sized {
    keys: [T; 4]
}

impl<K: Key, V> LeafLinear<K, V> {
    pub fn new() -> LeafLinear<K, V> {
        unimplemented!();
    }
}

impl<K: Key, V> JpmNode<K, V> for LeafLinear<K, V> {
    type OverflowNode = LeafBitmap<K, V>;
    fn get(&self, key: &[u8]) -> Option<&V> {
        unimplemented!();
    }

    fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        unimplemented!()
    }

    fn insert(&mut self, key: &[u8], value: V) -> InsertResult<V> {
        unimplemented!();
    }

    fn expand(self, key: &[u8], value: V) -> Box<LeafBitmap<K, V>> {
        unimplemented!();
    }
}
