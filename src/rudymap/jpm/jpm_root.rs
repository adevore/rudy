use rudymap::root_leaf::RootLeaf;
use super::innerptr::InnerPtr;
use super::traits::JpmNode;
use ::rudymap::results::InsertResult;
use ::Key;

pub struct JPM<K: Key, V> {
    head: InnerPtr<K, V>,
    len: usize
}

impl<K: Key, V> JPM<K, V> {
    pub fn new() -> JPM<K, V> {
        JPM {
            head: InnerPtr::empty(),
            len: 0
        }
    }
}

impl<K: Key, V> RootLeaf<K, V> for JPM<K, V> {
    type OverflowNode = JPM<K, V>;
    fn get(&self, key: K) -> Option<&V> {
        let bytes = key.into_bytes();
        self.head.get(bytes.as_ref())
    }

    fn insert(&mut self, key: K, value: V) -> InsertResult<V> {
        let bytes = key.into_bytes();
        if let InsertResult::Resize(value) = self.head.insert(bytes.as_ref(), value) {
            self.head = self.head.take().expand(bytes.as_ref(), value);
        }
        InsertResult::Success
    }

    fn expand(mut self, key: K, value: V) -> JPM<K, V> {
        let bytes = key.into_bytes();
        if let InsertResult::Resize(_) = self.head.insert(bytes.as_ref(), value) {
            panic!("Insertion failed unexpectedly in JPM")
        }
        self
    }

    fn len(&self) -> usize {
        self.len
    }
}
