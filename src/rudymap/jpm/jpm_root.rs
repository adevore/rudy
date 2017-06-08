use rudymap::root_leaf::RootLeaf;
use super::innerptr::InnerPtr;
use super::traits::JpmNode;
use ::rudymap::results::InsertResult;
use ::rudymap::rootptr::RootPtr;
use ::Key;

pub struct Jpm<K: Key, V> {
    head: InnerPtr<K, V>,
    len: usize
}

impl<K: Key, V> Jpm<K, V> {
    pub fn new() -> Jpm<K, V> {
        Jpm {
            head: InnerPtr::empty(),
            len: 0
        }
    }
}

impl<K: Key, V> RootLeaf<K, V> for Jpm<K, V> {
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

    fn expand(mut self: Box<Self>, key: K, value: V) -> RootPtr<K, V> {
        self.insert(key, value).success();
        self.into()
    }

    fn len(&self) -> usize {
        self.len
    }
}
