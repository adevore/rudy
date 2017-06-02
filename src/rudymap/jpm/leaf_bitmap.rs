use super::traits::JpmNode;
use super::innerptr::InnerPtr;
use ::rudymap::results::InsertResult;
use ::Key;

pub struct LeafBitmap<K: Key, V> {
    // Placeholders
    key: Option<K>,
    value: Option<V>
}

impl<K: Key, V> LeafBitmap<K, V> {
    pub fn new() -> LeafBitmap<K, V> {
        LeafBitmap {
            key: None,
            value: None
        }
    }
}

impl<K: Key, V> JpmNode<K, V> for LeafBitmap<K, V> {
    type OverflowNode = LeafBitmap<K, V>;
    fn get(&self, key: &[u8]) -> Option<&V> {
        unimplemented!();
    }

    fn insert(&mut self, key: &[u8], value: V) -> InsertResult<V> {
        unimplemented!();
    }

    fn expand(self, key: &[u8], value: V) -> Box<LeafBitmap<K, V>> {
        unimplemented!();
    }
}
